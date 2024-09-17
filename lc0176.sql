-- LeetCode problem 0176: Second Highest Salary
-- https://leetcode.com/problems/second-highest-salary/description/

select (
    select distinct salary
    from employee
    order by salary desc
    limit 1
    offset 1
) as SecondHighestSalary;
