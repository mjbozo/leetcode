-- LeetCode problem 0181: Employees Earning More Than Their Managers
-- https://leetcode.com/problems/employees-earning-more-than-their-managers/description/

select e1.name as Employee
from employee e1
inner join employee e2 on e1.managerId = e2.id
where e1.salary > e2.salary;
