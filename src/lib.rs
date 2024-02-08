mod solutions_1_30;
//  void combSum(int i,int target,vector<int>& arr,vector<int>& comb,vector<vector<int>>& ans,int n)
// {
//     if(i==n)
//     {
//         return;
//     }
//     if(target==0)
//     {
//         ans.push_back(comb);
//         return;
//     }
//     if(arr[i]<=target)
//     {
//         comb.push_back(arr[i]);
//         combSum(i,target-arr[i],arr,comb,ans,n);
//         comb.pop_back();
//     }
//     combSum(i+1,target,arr,comb,ans,n);
// }
// vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
//     vector<vector<int>> ans;
//     vector<int> comb;
//     int n=candidates.size();
//     combSum(0,target,candidates,comb,ans,n);
//     return ans;
// }
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut stack: Vec<i32> = Vec::new();

    fn dfs(array: &[i32], target: i32, stack: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(stack.clone());
        }
        if target <= 0 || array.is_empty() {
            return;
        }
        let n = array[0];
        println!("{array:?}");
        stack.push(n);
        dfs(array, target - n, stack, result);
        stack.pop();
        dfs(&array[1..], target, stack, result);
    }

    dfs(&candidates, target, &mut stack, &mut result);

    result
}

#[test]
fn test_fn() {
    assert_eq!(
        combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
    // assert_eq!(
    //     combination_sum(vec![2, 3, 5], 8),
    //     vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    // );
    // assert_eq!(combination_sum(vec![2], 1), []);
}
