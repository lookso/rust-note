// 返回教师的总数
pub fn num() -> i32 {
    5
}

// 计算一个教师平均教授的学生数量
pub fn average_students() -> i32 {
    use crate::school::student::num as student_num;
    student_num() / num()
}
