class Solution:
    def get_digits(self, n) -> list:
        digits = []
        while n > 0:
            digits.append(n % 10)
            n //= 10
        return digits
    
    def isHappy(self, n: int) -> bool:
        seen = set()
        while n != 1 and n not in seen:
            seen.add(n)
            digits:list = self.get_digits(n)
            n = sum(map(lambda x: x**2, digits))
        return n == 1
            



if __name__ == '__main__':
    assert Solution().isHappy(19)
    assert not Solution().isHappy(2)
    print(Solution().isHappy(19))
    print(Solution().isHappy(2))