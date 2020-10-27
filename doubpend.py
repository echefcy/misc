import arcade
import math

SCREEN_WIDTH = 1000
SCREEN_HEIGHT = 1000
SCREEN_TITLE = "Title"
LENGTH1 = 300
LENGTH2 = 300
MASS1 = 10
MASS2 = 10
GRAV_CONST = 1

def draw(_delta_time):
    
    draw.theta1 += draw.vel1
    draw.theta2 += draw.vel2

    x1 = LENGTH1*math.sin(draw.theta1) + SCREEN_WIDTH/2
    y1 = -LENGTH1*math.cos(draw.theta1) + SCREEN_HEIGHT/2

    x2 = x1 + LENGTH2*math.sin(draw.theta2)
    y2 = y1 - LENGTH2*math.cos(draw.theta2)

    arcade.start_render()
    arcade.draw_line(SCREEN_WIDTH/2,SCREEN_HEIGHT/2,x1,y1,arcade.color.BLACK)
    arcade.draw_circle_filled(x1,y1,20,arcade.color.BLACK)
    arcade.draw_line(x1,y1,x2,y2,arcade.color.BLACK)
    arcade.draw_circle_filled(x2,y2,20,arcade.color.BLACK)
    draw.accel1 = (-GRAV_CONST*(2*MASS1+MASS2)*math.sin(draw.theta1)-MASS2*GRAV_CONST*math.sin(draw.theta1-2*draw.theta2)-2*math.sin(draw.theta1-draw.theta2)*MASS2*(draw.vel2**2*LENGTH2+draw.vel1**2*LENGTH1*math.cos(draw.theta1-draw.theta2)))/(LENGTH1*(2*MASS1+MASS2-MASS2*math.cos(2*draw.theta1-2*draw.theta2)))
    draw.accel2 = (2*math.sin(draw.theta1-draw.theta2)*(draw.vel1**2*LENGTH1*(MASS1+MASS2)+GRAV_CONST*(MASS1+MASS2)*math.cos(draw.theta1)+draw.vel2**2*LENGTH2*MASS2*math.cos(draw.theta1-draw.theta2)))/(LENGTH2*(2*MASS1+MASS2-MASS2*math.cos(2*draw.theta1-2*draw.theta2)))
    draw.vel1 += draw.accel1
    draw.vel2 += draw.accel2

draw.theta1 = math.pi
draw.theta2 = math.pi/2
draw.vel1 = 0
draw.vel2 = 0

def main():
    arcade.open_window(SCREEN_WIDTH, SCREEN_HEIGHT,SCREEN_TITLE)
    arcade.set_background_color(arcade.color.WHITE)

    arcade.schedule(draw, 1/60)

    arcade.run()

    arcade.close_window()

if __name__ == "__main__":
    main()
