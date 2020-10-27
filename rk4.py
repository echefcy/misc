def rk4(f, initial_cond, h, n):
    if n==0:
        return initial_cond
    else:
        k1 = h*f(initial_cond[0],initial_cond[1])
        k2 = h*f(initial_cond[0]+h/2,initial_cond[1]+k1/2)
        k3 = h*f(initial_cond[0]+h/2,initial_cond[1]+k2/2)
        k4 = h*f(initial_cond[0]+h,initial_cond[1]+k3)
        tup = (initial_cond[0]+h,initial_cond[1]+(1/6)*(k1+2*k2+2*k3+k4))
        return rk4(f, tup, h, n-1)

# eq = lambda x,y : 2*x + y
# print(rk4(eq,(2,3),0.1,3))
