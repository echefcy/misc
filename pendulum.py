import arcade
import math

SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600
SCREEN_TITLE = "Title"
STRING_LENGTH = 300
CIRCLE_RADIUS = 40
GRAV_CONST = -2

def draw(_delta_time):
    
    draw.vel += draw.accel
    draw.theta += draw.vel

    x = STRING_LENGTH*math.cos(draw.theta+3*math.pi/2) + SCREEN_WIDTH/2
    y = STRING_LENGTH*math.sin(draw.theta+3*math.pi/2) + SCREEN_HEIGHT

    arcade.start_render()
    arcade.draw_line(SCREEN_WIDTH/2,SCREEN_HEIGHT,x,y,arcade.color.BLACK)
    arcade.draw_circle_filled(x,y,20,arcade.color.BLACK)
    draw.accel = GRAV_CONST*(1/(STRING_LENGTH))*math.sin(draw.theta)
    # print(math.sqrt((x-400)**2+(y-600)**2))
    


draw.theta = math.pi/4
draw.vel = 0
draw.accel = GRAV_CONST*(1/(STRING_LENGTH))*math.sin(draw.theta)

def main():
    arcade.open_window(SCREEN_WIDTH, SCREEN_HEIGHT,SCREEN_TITLE)
    arcade.set_background_color(arcade.color.WHITE)

    arcade.schedule(draw, 1/60)

    arcade.run()

    arcade.close_window()

if __name__ == "__main__":
    main()
