m ={"one": 1, "two":2, "three":3, "four":4, "five":5, "six":6, "seven":7, "eight":8, "nine":9}


with open("input.txt", 'r') as f:
    input_t = f.read().splitlines()

    r = 0
    

    for line in input_t:
        first = None
        sec = None
        s = ""
        for c in line:
            digit = None

            if c.isdigit():
                digit = int(c)
            else:
                s +=c
                for k,v in m.items():
                    if s.endswith(k):
                        digit = v
            
            if digit is not None:
                sec = int(digit)
                if first is None:
                    first = digit
        r += first * 10 + sec

print(r)




    

