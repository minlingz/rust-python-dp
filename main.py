import time

start_time = time.time()
for j in range(10000):
    sum = 0
    for i in range(10000):
        sum += i
    for i in range(10000):
        sum -= i
end_time = time.time()
elapsed_time = end_time - start_time

print(f"Result: {sum}")
print(f"Python elapsed time: {elapsed_time:.2f} seconds")
