# 分治

## lc 395 Longest Substring with At Least K Repeating Characters

Given a string s and an integer k, return the length of the longest substring of s such that the frequency of each character in this substring is greater than or equal to k.

**Example 1:**

```
Input: s = "aaabb", k = 3
Output: 3
Explanation: The longest substring is "aaa", as 'a' is repeated 3 times.
```

**Example 2:**

```
Input: s = "ababbc", k = 2
Output: 5
Explanation: The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.
```

**Constraints:**
```
1 <= s.length <= 10^4
s consists of only lowercase English letters.
1 <= k <= 10^5
```

### 题目大意
给你一个字符串 s 和一个整数 k ，请你找出 s 中的最长子串， 要求该子串中的每一字符出现次数都不少于 k 。返回这一子串的长度。

### 解题思路
**分治：**
由于题目要求子串中每一字符出现次数都不少于 k。因此包含少于 k 个字符的字符子串都不符合要求，需要将其剔除并进行递归，最终返回子串的最大值。