use crate::db::DBBackend;

use crate::db::tests::open_test_db;

use crate::task::test_utils::{example_task_1, example_task_break_1, arb_task_list};

// These tests don't really do anything. They just check that "if you add a valid task, the db does
// not error."
//
// TODO add an invalid task (eg null bytes in string) and check that it errors? there are kind of
// already tests for this in db::tests::list.rs
//
// Alternatively, pass through the number of rows changed from the sql calls and check that it's
// correct.

#[test]
fn test_db_add() {
    let mut db = open_test_db();
    let tx = db.transaction().expect("Failed to begin transaction");

    let task = example_task_1();
    let reward = example_task_break_1();

    let res = tx.add_task(&task);
    assert!(res.is_ok(), "Adding task failed: {:?}, err: {}", task, res.unwrap_err());
    let res = tx.add_task(&reward);
    assert!(res.is_ok(), "Adding task with break failed: {:?}, err: {}", reward, res.unwrap_err());
}

#[test]
fn test_db_add_task_rollback() {
    let task = example_task_1();

    let mut db = open_test_db();
    let tx = db.transaction().unwrap();

    let res = tx.add_task(&task);
    assert!(res.is_ok(), "Adding task failed: {}", res.unwrap_err());

    let res = tx.transaction.rollback();
    assert!(res.is_ok(), "Rolling back transaction failed: {}", res.unwrap_err());
}

#[test]
fn test_db_add_task_commit() {
    let task = example_task_1();

    let mut db = open_test_db();
    let tx = db.transaction().unwrap();

    let res = tx.add_task(&task);
    assert!(res.is_ok(), "Adding task failed: {}", res.unwrap_err());

    let res = tx.transaction.commit();
    assert!(res.is_ok(), "Committing transaction failed: {}", res.unwrap_err());
}

#[test]
fn test_db_add_duplicate_uuid_error() {
    let mut db = open_test_db();
    let tx = db.transaction().unwrap();

    let task = example_task_1();
    // we insert the same task twice
    tx.add_task(&task).expect("Adding task failed");

    // the second time it should fail
    let res = tx.add_task(&task);
    assert!(res.is_err(), "Adding the same task twice (with the same UUID) did not result in an error.");
 
    let err = res.unwrap_err();
    assert!(err.to_string().contains("UNIQUE constraint failed: tasks.uuid"),
            "Incorrect error message when inserting duplicate task: got {}", err);
}

proptest! {
    #[test]
    fn test_db_add_task_rollback_arb(tasks in arb_task_list()) {
        let mut db = open_test_db();
        let tx = db.transaction().unwrap();

        for task in tasks {
            let res = tx.add_task(&task);
            prop_assert!(res.is_ok(), "Adding task failed: {}", res.unwrap_err());
        }
        let res = tx.transaction.rollback();
        prop_assert!(res.is_ok(), "Rolling back transaction failed: {}", res.unwrap_err());
    }
}

proptest! {
    #[test]
    fn test_db_add_task_commit_arb(tasks in arb_task_list()) {
        let mut db = open_test_db();
        let tx = db.transaction().unwrap();

        for task in tasks {
            let res = tx.add_task(&task);
            prop_assert!(res.is_ok(), "Adding task failed: {}", res.unwrap_err());
        }
        let res = tx.transaction.commit();
        prop_assert!(res.is_ok(), "Committing transaction failed: {}", res.unwrap_err());
    }
}
