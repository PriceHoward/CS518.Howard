#include <algorithm>
#include <concepts>
#include <iostream>
#include <string>
#include <unordered_set>
#include <vector>

template <typename T>
concept Hashable = requires(T x) {
  { std::hash<T>{}(x) } -> std::convertible_to<size_t>;
};

/*
    This stable_dedup is O(n) due to the unordered_set. The unordered set only
   makes us loop once through the vector.

    The trade off for having hashing is the better time complexity of O(n). We
   dont have to rely on going through the list twice. We also dont have to
   create an overloaded == to be able to check the eq of the values to be able
   to see if there are double inside of the vector.

*/
template <typename T>
  requires Hashable<T>
std::vector<T> stable_dedup(const std::vector<T> &xs) {
  std::unordered_set<T> seen;
  std::vector<T> result;
  for (const auto &x : xs) {
    if (seen.insert(x).second)
      result.push_back(x);
  }
  return result;
}

/*
This stable_dedup_eq is O(n^2) this is because we have the outside loop of the
   for loop. then we have the .find which goes through the vector again
   searching for any duplicates.

    The Trade off for not having hashing is it is more time complex. Where we go
   from O(n) to O(n^2). This is due to having the double loop and not being able
   to use the unordered_set.
*/

template <typename T> std::vector<T> stable_dedup(const std::vector<T> &xs) {
  std::vector<T> result;
  for (const auto &x : xs) {
    if (std::find(result.begin(), result.end(), x) == result.end())
      result.push_back(x);
  }
  return result;
}

struct NonHashType {
  int val1, val2;
  bool operator==(const NonHashType &compareStruct) const {
    return val1 == compareStruct.val1 && val2 == compareStruct.val2;
  }
};

int main() {
  // Hashable Types by default.
  std::vector<int> intVec = {5, 4, 2, 1, 6, 4, 3, 2, 1, 7, 6, 5, 4, 3, 2, 1};
  std::vector<std::string> stringVec = {"Hello", "Bonjour", "Hello",
                                        "Adios", "Wazup",   "Adios"};

  // Non hashable Type.
  std::vector<NonHashType> noHashVec = {{5, 5}, {1, 3}, {6, 5}, {5, 5},
                                        {1, 2}, {6, 5}, {1, 1}};

  std::cout << "-----------Integer Test-----------" << std::endl;
  for (int output : stable_dedup(intVec)) {
    std::cout << output << std::endl;
  }
  std::cout << "-----------String Test-----------" << std::endl;
  for (std::string output : stable_dedup(stringVec)) {
    std::cout << output << std::endl;
  }
  std::cout << "-----------NoHashType Test-----------" << std::endl;
  for (const NonHashType &output : stable_dedup(noHashVec)) {
    std::cout << "{" << output.val1 << "," << output.val2 << "}" << std::endl;
  }
}