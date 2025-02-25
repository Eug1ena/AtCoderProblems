use sql_client::contest_problem::ContestProblemClient;
use sql_client::models::ContestProblem;

mod utils;

fn create_problem(id: i32) -> ContestProblem {
    ContestProblem {
        contest_id: format!("contest{}", id),
        problem_id: format!("problem{}", id),
    }
}

#[async_std::test]
async fn test_contest_problem() {
    let pool = utils::initialize_and_connect_to_test_sql().await;
    assert!(pool.load_contest_problem().await.unwrap().is_empty());

    pool.insert_contest_problem(&[create_problem(1), create_problem(2)])
        .await
        .unwrap();
    assert_eq!(
        pool.load_contest_problem().await.unwrap(),
        vec![create_problem(1), create_problem(2)]
    );
    pool.insert_contest_problem(&[create_problem(1)])
        .await
        .unwrap();
    assert_eq!(
        pool.load_contest_problem().await.unwrap(),
        vec![create_problem(1), create_problem(2)]
    );
}
