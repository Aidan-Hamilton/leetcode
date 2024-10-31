# TODO

- **Use clap top parse cli arguments**
  - Implement clean or remove which will compress solutions into an archive
- **Auto-create test cases in code**
  - Refer to the [LeetCode guide on creating test cases](https://support.leetcode.com/hc/en-us/articles/32442719377939-How-to-create-test-cases-on-LeetCode) for more information.
- **Auto generation of solution list (when 100 problems solved)**
- **Use cargo aliases/tools to simplify the cli**
  - Can be done using unittests
- **Add ability to autofetch graphql request to prevent legal issues** (GraphQL folder contains reverse engineered GraphQL requests which may violate TOS or be copyrighted)
  - A way around legal issues is by not hosting publically but fetch the GraphQL requests(through build.rs)
