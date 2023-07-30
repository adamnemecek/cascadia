// https://github.com/RobinSchmidt/RS-MET/blob/6c01cbaad7cce3daa3293c444dd9e4b74e5ebfbe/Misc/UnusedCode/RSLibAndTests/RSLib/Code/Math/Functions/IntegerFunctions.cpp#L131
// int rsLeviCivita(int indices[], int N)
// {
//   int result = 1;
//   for(int i = 0; i < N-1; i++)
//   {
//     for(int j = i+1; j < N; j++)
//     {
//       int d = indices[j] - indices[i];
//       if( d == 0 )
//         return 0;
//       else if( d < 0 )
//         result *= -1;
//     }
//   }
//   return result;
// }

pub fn levi_civita(ind: &[isize]) -> isize {
    let mut result = 1;
    for i in 0..ind.len() - 1 {
        for j in i + 1..ind.len() {
            let d = ind[j] - ind[i];
            if d == 0 {
                return 0;
            } else if d < 0 {
                result *= -1;
            }
        }
    }
    result
}

// https://github.com/RobinSchmidt/RS-MET/blob/6c01cbaad7cce3daa3293c444dd9e4b74e5ebfbe/Misc/UnusedCode/RSLibAndTests/RSLib/Code/Math/Functions/IntegerFunctions.cpp#L69C1-L73C4

// int rsGeneralizedDelta(int superscripts[], int subscripts[], int N)
// {
//   return rsLeviCivita(superscripts, N) * rsLeviCivita(subscripts, N);
//   // is this formula right? what if leviCivita returns 0 but the subscripts
// }

fn generalized_delta(
    superscript: &[isize],
    subscript: &[isize],
) -> isize {
    //
    levi_civita(superscript) * levi_civita(subscript)
}
