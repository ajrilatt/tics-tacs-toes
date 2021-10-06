# Adam Rilatt
# 09 / 03 / 20
# Tiny Tic Tac Toe

# This really shouldn't exist, but I love to prove a point. This is my tictactoe
# that has been condensed to 11 lines (technically less, but I obey an 80 col)
# limit. :)

b,t=[[col+row for row in range(3)] for col in range(0,9,3)],0
while 1:
    print(('O' if t%2 else 'X')+" TURN\n"+('\n'.join(
    ['-'*10 if r%2 else ' | '.join(map(str,b[r//2])) for r in range(5)])))
    s=int(input('> '))
    if s in list(range(9)) and str(b[s//3][s%3]) not in 'XO':
        t,b[s//3][s%3]=t+1,'O' if t%2 else 'X'
    if any(l[r[0]]==l[r[1]] and l[r[1]]==l[r[2]] for r in (
    (0,1,2),(3,4,5),(6,7,8),(0,3,6),(1,4,7),(2,5,8),(0,4,8),(2,4,6)
    ) for l in [sum(b,[])]) or t>8:break
print(('X' if t%2 else 'O')+" WINS!" if t<8 else "TIE!")
