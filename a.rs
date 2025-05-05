// Slip1
#include <GL/glut.h>
#include <stdio.h>

void BresLine(int xa, int ya, int xb, int yb, int type){
    int dx, dy, d;
    int c, r, f;
    dx = xb - xa;
    dy = yb - ya;
    int i = 0;
    printf("Type: %d", type);


    if(abs(dx) > abs(dy)){
        d = 2 * abs(dy) - abs(dx);
        if(dx > 0){
            c = xa;
            r = ya;
            f = xb;
        }else{
            c = xb;
            r = yb;
            f = xa;
        }
        while(f > c){
            if(d < 0){
                c++;
                d = d + 2 * abs(dy);
            }else{
                if(dx > 0 && dy > 0 || dx < 0 && dy < 0){
                    r++;
                }else{
                    r--;
                }
                c++;
                d = d + 2 * abs(dy) - 2 * abs(dx);
            }
            // glVertex2d(c, r);
               // General
        if(type == 0){
            glVertex2d(c, r);
        }    
    
        // Type: Dotted
        if(type == 1){
            if(i % 4 == 0){
                glVertex2d(c, r);
            }
        }

        // Type: Dashed
        if(type == 2){
            if(i % 10  < 5){
                glVertex2d(c, r);
            }
        }

        // Type: Dotted Dash
        if(type == 3){
            if(i % 15 < 5 || (i % 15 > 8 && i % 15 < 10)){
                glVertex2d(c, r);
            }
        }
        i++;
        }
    }else{
        d = 2 * abs(dx) - abs(dy);
        if(dy > 0){
            c = xa;
            r = ya;
            f = yb;
        }else{
            c = xb;
            r = yb;
            f = ya;
        }
        while(f > r){
            if(d < 0){
                r++;
                d = d + 2 * abs(dx);
            }else{
                if(dx > 0 && dy > 0 || dx < 0 && dy < 0){
                    c++;
                }else{
                    c--;
                }
                r++;
                d = d + 2 * abs(dx) - 2 * abs(dy);
            }
            // General
        if(type == 0){
            glVertex2d(c, r);
        }    
    
        // Type: Dotted
        if(type == 1){
            if(i % 4 == 0){
                glVertex2d(c, r);
            }
        }

        // Type: Dashed
        if(type == 2){
            if(i % 10  < 5){
                glVertex2d(c, r);
            }
        }

        // Type: Dotted Dash
        if(type == 3){
            if(i % 15 < 5 || (i % 15 > 8 && i % 15 < 10)){
                glVertex2d(c, r);
            }
        }
        i++;
        }
    }
}


void Draw(){
    glPointSize(2);
    glClear(GL_COLOR_BUFFER_BIT);
    glBegin(GL_POINTS);
    glVertex2d(-320, 100);
    // Window
    BresLine(-320, 0, 320, 0, 0);
    BresLine(0, -240, 0, 240, 0);
    BresLine(0, 0, -200, 100, 0);
    BresLine(0, 0, -200, 120, 1);
    BresLine(0, 0, -200, 140, 2);
    BresLine(0, 0, -200, 160, 3);    
    glEnd();
    glFlush();
}

int main(int argc, char **argv){
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
    glutInitWindowSize(640, 480);
    glutCreateWindow("1");
    glutInitWindowPosition(0, 0);
    glClearColor(1.0, 1.0, 1.0, 0);
    glColor3f(0, 0, 0);
    gluOrtho2D(-320, 320, -240, 240);
    glutDisplayFunc(Draw);
    glutMainLoop();
    return 0;
}

// Slip2
#include <GL/glut.h>
#include <stdio.h>
#include <stdbool.h>


bool firstClick = true;;
bool secondClick = false;


int xa, ya, xb, yb;


void BresLine(int xa, int ya, int xb, int yb){
   int dx, dy, d;
   int c, r, f;
   dx = xb - xa;
   dy = yb - ya;




   if(abs(dx) > abs(dy)){
       d = 2 * abs(dy) - abs(dx);
       if(dx > 0){
           c = xa;
           r = ya;
           f = xb;
       }else{
           c = xb;
           r = yb;
           f = xa;
       }
       while(f > c){
           if(d < 0){
               c++;
               d = d + 2 * abs(dy);
           }else{
               if(dx > 0 && dy > 0 || dx < 0 && dy < 0){
                   r++;
               }else{
                   r--;
               }
               c++;
               d = d + 2 * abs(dy) - 2 * abs(dx);
           }
           glVertex2d(c, r);
       }
   }else{
       d = 2 * abs(dx) - abs(dy);
       if(dy > 0){
           c = xa;
           r = ya;
           f = yb;
       }else{
           c = xb;
           r = yb;
           f = ya;
       }
       while(f > r){
           if(d < 0){
               r++;
               d = d + 2 * abs(dx);
           }else{
               if(dx > 0 && dy > 0 || dx < 0 && dy < 0){
                   c++;
               }else{
                   c--;
               }
               r++;
               d = d + 2 * abs(dx) - 2 * abs(dy);
           }
           glVertex2d(c, r);
       }
   }
}




void Draw(){
   glPointSize(2);
   glClear(GL_COLOR_BUFFER_BIT);
   glBegin(GL_POINTS);
   glVertex2d(-320, 100);


   if(firstClick){
           // Outer rec (standard rectangle)
           BresLine(xa, ya, xb, ya);
           BresLine(xb, ya, xb, yb);
           BresLine(xb, yb, xa, yb);
           BresLine(xa, yb, xa, ya);


           int midTopX = (xa + xb) / 2;
           int midTopY = ya;


           int midBottomX = (xa + xb) / 2;
           int midBottomY = yb;


           int midLeftX = xa;
           int midLeftY = (ya + yb) / 2;


           int midRightX = xb;
           int midRightY = (ya + yb) / 2;


           glVertex2d(midBottom);


           BresLine(midTopX, midTopY, midRightX, midRightY);
           BresLine(midRightX, midRightY, midBottomX, midBottomY);
           BresLine(midBottomX, midBottomY, midLeftX, midLeftY);
           BresLine(midLeftX, midLeftY, midTopX, midTopY);


           int innerX1 = xa + (xb - xa) / 4;
           int innerX2 = xb - (xb - xa) / 4;
           int innerY1 = ya + (yb - ya) / 4;
           int innerY2 = yb - (yb - ya) / 4;


           BresLine(innerX1, innerY1, innerX2, innerY1); // Top
           BresLine(innerX2, innerY1, innerX2, innerY2); // Right
           BresLine(innerX2, innerY2, innerX1, innerY2); // Bottom
           BresLine(innerX1, innerY2, innerX1, innerY1); // Left


   }




   glEnd();
   glFlush();
}


void Mouse(int b, int s, int x, int y){
   if(b == GLUT_LEFT_BUTTON && s == GLUT_DOWN){
       if(firstClick){
           xa = x - 320;
           ya = 240 - y;
           // printf("First point: (%d, %d)", xa, ya);
           firstClick = false;
           secondClick = false;
       }else{
           xb = x - 320;
           yb = 240 - y;
           // printf("Second point: (%d, %d)\n", xb, yb);
           secondClick = true;
           glutPostRedisplay();
           firstClick = true;
       }
   }
}


int main(int argc, char **argv){
   glutInit(&argc, argv);
   glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
   glutInitWindowSize(640, 480);
   glutCreateWindow("2");
   glutInitWindowPosition(0, 0);
   glClearColor(1.0, 1.0, 1.0, 0);
   glColor3f(0, 0, 0);
   gluOrtho2D(-320, 320, -240, 240);
   glutDisplayFunc(Draw);
   glutMouseFunc(Mouse);
   glutMainLoop();
   return 0;
}

// Slip3
#include<stdio.h>
#include<GL/glut.h>
#include<math.h>

int xa, ya, xb, yb, xc, yc, r;

int ROUND(float a){
  return (int) (a + 0.5);
}

void BresCircle(int xc, int yc, int r){
  int x, y, d;
  x = 0; y = r; d = 3-2*r;

  do{
  glVertex2d(xc+x,yc+y);
  glVertex2d(xc+y,yc+x);
  glVertex2d(xc-x,yc-y);
  glVertex2d(xc-y,yc-x);
  glVertex2d(xc+x,yc-y);
  glVertex2d(xc-x,yc+y);
  glVertex2d(xc-y,yc+x);
  glVertex2d(xc+y,yc-x);

    if(d<0){
      x = x+1;
      d = d+4*x+6;
    }

    else{
      x = x+1;
      y = y-1;
      d = d+4*x-4*y+10;
    }
  }
  while(y>=x);
}

void bresline(int xa, int ya, int xb, int yb){
    int c,r,f;
    int dx, dy, d;

    dx = xb - xa;
    dy = yb - ya;

    if(abs(dx)>abs(dy)){
        d = 2*abs(dy)-abs(dx);

        if(dx>0){
              c = xa;
              r = ya;
              f = xb;
                }
        else{
              c = xb;
              r = yb;
              f = xa;
            }
        while(f>c){
                    if(d<0){
                        c = c+1;
                        d = d+2*abs(dy);
                    }
                    else{
                        if(dx>0 && dy>0 || dx<0 && dy<0){
                            c = c+1;
                            r = r+1;
                        }
                        else{
                            c = c+1;
                            r = r-1;
                        }
                        d = d+2*abs(dy)- 2*abs(dx);
                    }
                glVertex2d(c,r);
                }

    }

    else{
        d = 2*abs(dx)-abs(dy);

        if(dy>0){
              c = xa;
              r = ya;
              f = yb;
                }
        else{
              c = xb;
              r = yb;
              f = ya;
            }
        while(f>r){
                    if(d<0){
                        r = r+1;
                        d = d+2*abs(dx);
                    }
                    else{
                        if(dx>0 && dy>0 || dx<0 && dy<0){
                            c = c+1;
                            r = r+1;
                        }
                        else{
                            c = c-1;
                            r = r+1;
                        }
                        d = d+2*abs(dx)- 2*abs(dy);
                    }
                glVertex2d(c,r);
                }
    }
}

void shape(){

    //Triangle
    glColor3f(0,0,0);
    bresline(xc,yc+r,xc+(r*0.866),yc-(r*0.5));
    bresline(xc,yc+r,xc-(r*0.866),yc-(r*0.5));
    bresline(xc-(r*0.866),yc-(r*0.5),xc+(r*0.866),yc-(r*0.5));

    glColor3f(0,0,1);
    BresCircle(xc, yc, r/2);
}

void Mouse(int button, int state, int x, int y){
    if (button == GLUT_LEFT_BUTTON && state == GLUT_DOWN){
            xc = x;
            yc = 720-y;
            glutPostRedisplay();
       }
}

void Draw(){
glClear(GL_COLOR_BUFFER_BIT);
glBegin(GL_POINTS);
shape();
glEnd();
glFlush();
}

int main(int argc ,char **argv){
printf("\nEnter Radius Length: ");
scanf("%d", &r);

glutInit(&argc, argv);
glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
glutInitWindowPosition(0,0);
glutInitWindowSize(1080,720);
glutCreateWindow("WINDOW");
glClearColor(1,1,1,0);
gluOrtho2D(0,1080,0,720);
glutDisplayFunc(Draw);
glutMouseFunc(Mouse);
glutMainLoop();
return 0;
}

// Slip4
#include<stdio.h>
#include<GL/glut.h>
#include<math.h>

int xa, ya, xb, yb, xc, yc, r;
int clickcount = 0;

int ROUND(float a){
  return (int) (a + 0.5);
}

void BresCircle(int xc, int yc, int r){
  int x, y, d;
  x = 0; y = r; d = 3-2*r;

  do{
  glVertex2d(xc+x,yc+y);
  glVertex2d(xc+y,yc+x);
  glVertex2d(xc-x,yc-y);
  glVertex2d(xc-y,yc-x);
  glVertex2d(xc+x,yc-y);
  glVertex2d(xc-x,yc+y);
  glVertex2d(xc-y,yc+x);
  glVertex2d(xc+y,yc-x);

    if(d<0){
      x = x+1;
      d = d+4*x+6;
    }

    else{
      x = x+1;
      y = y-1;
      d = d+4*x-4*y+10;
    }
  }
  while(y>=x);
}

void bresline(int xa, int ya, int xb, int yb){
    int c,r,f;
    int dx, dy, d;

    dx = xb - xa;
    dy = yb - ya;

    if(abs(dx)>abs(dy)){
        d = 2*abs(dy)-abs(dx);

        if(dx>0){
              c = xa;
              r = ya;
              f = xb;
                }
        else{
              c = xb;
              r = yb;
              f = xa;
            }
        while(f>c){
                    if(d<0){
                        c = c+1;
                        d = d+2*abs(dy);
                    }
                    else{
                        if(dx>0 && dy>0 || dx<0 && dy<0){
                            c = c+1;
                            r = r+1;
                        }
                        else{
                            c = c+1;
                            r = r-1;
                        }
                        d = d+2*abs(dy)- 2*abs(dx);
                    }
                glVertex2d(c,r);
                }

    }

    else{
        d = 2*abs(dx)-abs(dy);

        if(dy>0){
              c = xa;
              r = ya;
              f = yb;
                }
        else{
              c = xb;
              r = yb;
              f = ya;
            }
        while(f>r){
                    if(d<0){
                        r = r+1;
                        d = d+2*abs(dx);
                    }
                    else{
                        if(dx>0 && dy>0 || dx<0 && dy<0){
                            c = c+1;
                            r = r+1;
                        }
                        else{
                            c = c-1;
                            r = r+1;
                        }
                        d = d+2*abs(dx)- 2*abs(dy);
                    }
                glVertex2d(c,r);
                }
    }
}

void shape(){
    int xm, ym, xdr, ydr;
    float r;

    xm = (xa + xb) / 2;
    ym = (ya + yb) / 2;

    // Outer rectangle
    bresline(xa, ya, xb, ya);
    bresline(xb, ya, xb, yb);
    bresline(xb, yb, xa, yb);
    bresline(xa, yb, xa, ya);

    // Diamond inside the rectangle
    bresline((xa + xb) / 2, ya, xb, (ya + yb) / 2);
    bresline(xb, (ya + yb) / 2, (xa + xb) / 2, yb);
    bresline((xa + xb) / 2, yb, xa, (ya + yb) / 2);
    bresline(xa, (ya + yb) / 2, (xa + xb) / 2, ya);

    xdr = ((xa + xb)/2 + xb) / 2;
    ydr = (ya + (ya + yb)/2) / 2;

    r = sqrt((xdr - xm)*(xdr - xm) + (ydr - ym)*(ydr - ym));

    BresCircle(xm, ym, (int)r);
}

void Mouse(int button, int state, int x, int y){
    if (button == GLUT_LEFT_BUTTON && state == GLUT_DOWN){
        if(clickcount == 0){
            xa = x;
            ya = 720-y;
            clickcount = 1;
        }
        else {
            xb = x;
            yb = 720-y;
            clickcount = 0;

            glutPostRedisplay();
        }
    }
}

void Draw(){
glClear(GL_COLOR_BUFFER_BIT);
glBegin(GL_POINTS);
shape();
glEnd();
glFlush();
}

int main(int argc ,char **argv){
glutInit(&argc, argv);
glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
glutInitWindowPosition(0,0);
glutInitWindowSize(1080,720);
glutCreateWindow("WINDOW");
glClearColor(1,1,1,0);
glColor3f(0,0,0);
gluOrtho2D(0,1080,0,720);
glutDisplayFunc(Draw);
glutMouseFunc(Mouse);
glutMainLoop();
return 0;
}

// Slip5
#include<stdio.h>
#include<GL/glut.h>
#include<math.h>

int xa, ya, xb, yb, xc, yc, r;
int clickcount = 0;

int ROUND(float a){
  return (int) (a + 0.5);
}

void BresCircle(int xc, int yc, int r){
  int x, y, d;
  x = 0; y = r; d = 3-2*r;

  do{
  glVertex2d(xc+x,yc+y);
  glVertex2d(xc+y,yc+x);
  glVertex2d(xc-x,yc-y);
  glVertex2d(xc-y,yc-x);
  glVertex2d(xc+x,yc-y);
  glVertex2d(xc-x,yc+y);
  glVertex2d(xc-y,yc+x);
  glVertex2d(xc+y,yc-x);

    if(d<0){
      x = x+1;
      d = d+4*x+6;
    }

    else{
      x = x+1;
      y = y-1;
      d = d+4*x-4*y+10;
    }
  }
  while(y>=x);
}

void bresline(int xa, int ya, int xb, int yb){
    int c,r,f;
    int dx, dy, d;

    dx = xb - xa;
    dy = yb - ya;

    if(abs(dx)>abs(dy)){
        d = 2*abs(dy)-abs(dx);

        if(dx>0){
              c = xa;
              r = ya;
              f = xb;
                }
        else{
              c = xb;
              r = yb;
              f = xa;
            }
        while(f>c){
                    if(d<0){
                        c = c+1;
                        d = d+2*abs(dy);
                    }
                    else{
                        if(dx>0 && dy>0 || dx<0 && dy<0){
                            c = c+1;
                            r = r+1;
                        }
                        else{
                            c = c+1;
                            r = r-1;
                        }
                        d = d+2*abs(dy)- 2*abs(dx);
                    }
                glVertex2d(c,r);
                }

    }

    else{
        d = 2*abs(dx)-abs(dy);

        if(dy>0){
              c = xa;
              r = ya;
              f = yb;
                }
        else{
              c = xb;
              r = yb;
              f = ya;
            }
        while(f>r){
                    if(d<0){
                        r = r+1;
                        d = d+2*abs(dx);
                    }
                    else{
                        if(dx>0 && dy>0 || dx<0 && dy<0){
                            c = c+1;
                            r = r+1;
                        }
                        else{
                            c = c-1;
                            r = r+1;
                        }
                        d = d+2*abs(dx)- 2*abs(dy);
                    }
                glVertex2d(c,r);
                }
    }
}

void shape(){
    int xi,yi;
    xi = (xb-xa)/4;
    yi = (yb-ya)/4;
    int x = xa; int y = ya;

    for(int i=0; i<5;i++){
        bresline(xa,y,xb,y);
        bresline(x,ya,x,yb);
        x = x+xi; y = y+yi;
    }
}

void Mouse(int button, int state, int x, int y){
    if (button == GLUT_LEFT_BUTTON && state == GLUT_DOWN){
        if(clickcount == 0){
            xa = x;
            ya = 720-y;
            clickcount = 1;
        }
        else {
            xb = x;
            yb = 720-y;
            clickcount = 0;

            glutPostRedisplay();
        }
    }
}

void Draw(){
glClear(GL_COLOR_BUFFER_BIT);
glBegin(GL_POINTS);
shape();
glEnd();
glFlush();
}

int main(int argc ,char **argv){
glutInit(&argc, argv);
glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
glutInitWindowPosition(0,0);
glutInitWindowSize(1080,720);
glutCreateWindow("WINDOW");
glClearColor(1,1,1,0);
glColor3f(0,0,0);
gluOrtho2D(0,1080,0,720);
glutDisplayFunc(Draw);
glutMouseFunc(Mouse);
glutMainLoop();
return 0;
}

// Slip6
#include <GL/glut.h>
#include <stdio.h>
#include <math.h>

int clickCount = 0;
float ctrlPoints[2][4]; // P0, P1, P2, P3

int ROUND(float a){
  return (int) (a+0.5); 
}

void DDA(int xa,int ya, int xb, int yb){
  int dx,dy,steps;
  dx=xb-xa;
  dy=yb-ya;
  if(abs(dx)>abs(dy)){
    steps=abs(dx);
  }
  else{
    steps=abs(dy);
  }
  float xinc,yinc;
  xinc=(float)dx/steps;
  yinc=(float)dy/steps;
  
  glVertex2d(xa,ya);
  float x=xa,y=ya;
  for(int i=0;i<steps;i++){
    x=x+xinc;
    y=y+yinc;
    glVertex2d(ROUND(x),ROUND(y));
  }
}
void Bezier() {
    float x, y, t;
    glColor3f(1, 0, 0); // Red curve

    for (t = 0.0; t <= 1.0; t += 0.001) {
        if (clickCount == 2) {
            // Linear Bézier
            x = (1 - t) * ctrlPoints[0][0] + t * ctrlPoints[0][1];
            y = (1 - t) * ctrlPoints[1][0] + t * ctrlPoints[1][1];
        }
        else if(clickCount==3)
         {
        x = pow(1 - t, 2) * ctrlPoints[0][0] +
            2 * (1 - t) * t * ctrlPoints[0][1] +
            pow(t, 2) * ctrlPoints[0][2];

        y = pow(1 - t, 2) * ctrlPoints[1][0] +
            2 * (1 - t) * t * ctrlPoints[1][1] +
            pow(t, 2) * ctrlPoints[1][2];
        }
        else if(clickCount==4)
         {
            x = pow(1 - t, 3) * ctrlPoints[0][0] +
            3 * pow(1 - t, 2) * t * ctrlPoints[0][1] +
            3 * (1 - t) * pow(t, 2) * ctrlPoints[0][2] +
            pow(t, 3) * ctrlPoints[0][3];

            y = pow(1 - t, 3) * ctrlPoints[1][0] +
            3 * pow(1 - t, 2) * t * ctrlPoints[1][1] +
            3 * (1 - t) * pow(t, 2) * ctrlPoints[1][2] +
            pow(t, 3) * ctrlPoints[1][3];
         }
         glVertex2d(ROUND(x), ROUND(y));
    }
}


void display() {
    glClear(GL_COLOR_BUFFER_BIT);
    glBegin(GL_POINTS);

  
        glColor3f(0, 0, 1); // Blue control polygon
        for(int i=0;i<clickCount-1;i++)
        {
        DDA(ctrlPoints[0][i], ctrlPoints[1][i], ctrlPoints[0][i+1], ctrlPoints[1][i+1]);
        }
        Bezier();


    glEnd();
    glFlush();
}

void mouse(int button, int state, int x, int y) {
    if (button == GLUT_LEFT_BUTTON && state == GLUT_DOWN) {
        if (clickCount < 4) {
            ctrlPoints[0][clickCount] = x;
            ctrlPoints[1][clickCount] = 480 - y; // Flip Y
            clickCount++;
                glutPostRedisplay();
            
        }
    }
}

void keyboard(unsigned char key, int x, int y) {
    if (key == 'r' || key == 'R') {
        clickCount = 0;
        glutPostRedisplay();
    }
}

int main(int argc, char **argv) {
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
    glutInitWindowSize(640, 480);
    glutInitWindowPosition(100, 100);
    glutCreateWindow("Quadratic & Cubic Bezier Curve with DDA");

    glClearColor(1, 1, 1, 0);
    glColor3f(0, 0, 0);
    gluOrtho2D(0, 640, 0, 480);

    glutDisplayFunc(display);
    glutMouseFunc(mouse);
    glutKeyboardFunc(keyboard);
    glutMainLoop();
    return 0;
}

// #define GL_SILENCE_DEPRECATION
#include <GL/glut.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>


int xa,ya,xb,yb;
int steps=5;


int Round(float a)
{
    return(int)(a+0.5);
}

void ddaline(int xa,int ya,int xb,int yb)
{
    int dx,dy,steps;
    dx = xb - xa;
    dy = yb - ya;
    
    if( abs(dx) > abs(dy))
    {
      steps = abs(dx);
    }
    else
    {
      steps = abs(dy);
    }
    
    float xinc , yinc , x, y;
    xinc = (float) dx / steps ;
    yinc = (float) dy / steps ;
    x = xa;
    y = ya;
    glVertex2d(xa,ya);
    
    for(int i = 0;i <= steps; i++ )
    {
      x = x + xinc;
      y = y + yinc;
      glVertex2d(Round(x),Round(y));
    }
}

void koch_curve(float xa, float ya, float xb, float yb,int steps){
    if(steps == 0){
        ddaline(xa,ya,xb,yb);
        return;
    }
    
    float x1,y1,x2,y2,r_dx,r_dy,x3,y3, dx, dy;
    x1 = xa + (xb-xa)/3;
    y1 = ya + (yb-ya)/3;
    x2 = xa + 2*(xb-xa)/3;
    y2 = ya + 2*(yb-ya)/3;
    dx = x2-x1;
    dy = y2-y1;
    float angle = M_PI/3;
    r_dx = dx*cos(angle) - dy*sin(angle);
    r_dy = dx*sin(angle) + dy*cos(angle);
    x3 = x1 + r_dx;
    y3 = y1 + r_dy;

    

    koch_curve(xa,ya,x1,y1,steps-1);
    koch_curve(x1,y1,x3,y3,steps-1);
    koch_curve(x3,y3,x2,y2,steps-1);
    koch_curve(x2,y2,xb,yb,steps-1);
}

void draw(){
    glClear(GL_COLOR_BUFFER_BIT);
    glBegin(GL_POINTS);
    
    koch_curve(100,100,400,100,steps);
    koch_curve(400,100,250,400,steps);
    koch_curve(250,400,100,100,steps);
    glEnd();
    glFlush();

}

int main(int argc, char ** argv){
    
  glutInit(&argc, argv);
  glutInitWindowPosition(0,0);
  glutInitWindowSize(640,480);
  glutCreateWindow("Clipping");
  glClearColor(0,0,0,0 );
  glColor3f(1.0,1.0,1.0);
  gluOrtho2D(0,640,0,480);
  glutDisplayFunc(draw);
  glutMainLoop();
}

// Slip7,8,9
#include<GL/glut.h>
#include<stdio.h>
#include<stdlib.h>
#include<math.h>

int count=0,v;
int input[3][10];
float output[3][10],tr[3][3],rotate[3][3],scale[3][3],ref[3][3],xshear[3][3],yshear[3][3];

void mul(int input[3][10],float tr[3][3],float output[3][10],int v)
{
      for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              output[i][j]=0;
              for(int k=0;k<3;k++)
              {
                  output[i][j] += tr[i][k] * input[k][j];
              }
          }
      }
}

void Translation()
{
      for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              printf("\t%d",input[i][j]);
          }
          printf("\n");
      }
      
      int tx,ty;
      printf("\nEnter Value of TX and TY: ");
      scanf("%d%d",&tx,&ty);
      for(int i=0;i<3;i++)
      {
          for(int j=0;j<3;j++)
          {
              if(i==j)
              {
                  tr[i][j]=1;
              }
              else
              {
                  tr[i][j]=0;
              }
          }
      }
      
      tr[0][2]=tx;
      tr[1][2]=ty;
      
      for(int i=0;i<3;i++)
      {
          for(int j=0;j<3;j++)
          {
              printf("\t%f",tr[i][j]);
          }
          printf("\n");
      }
      
      mul(input,tr,output,v);
        for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              printf("\t%f",output[i][j]);
          }
          printf("\n");
      }
      
      
}
float angle(float thita)
{
    return  (thita*3.14/180);
}


void Rotation()
{
     for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              printf("\t%d",input[i][j]);
          }
          printf("\n");
      }
      float xr,yr,thita;
      	printf("Enter Value of Fixed Point: ");
	scanf("%f%f",&xr,&yr);
	printf("Enter value of Angle: ");
	scanf("%f",&thita);
	
	float sinT=sin(angle(thita));
	float cosT=cos(angle(thita));
      
      
      	rotate[0][0]=cosT;
	rotate[1][0]=sinT;
	rotate[2][0]=0;
	
	rotate[0][1]=-sinT;
	rotate[1][1]=cosT;
	rotate[2][1]=0;
	
	rotate[0][2]=xr*(1-cosT)+yr*sinT;
	rotate[1][2]=yr*(1-cosT)-xr*sinT;
	rotate[2][2]=1;
	
	mul(input,rotate,output,v);
        for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              printf("\t%f",output[i][j]);
          }
          printf("\n");
      }
}

void Scaling()
{
        for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              printf("\t%d",input[i][j]);
          }
          printf("\n");
      }
      int xf,yf;
    float sx,sy;
    printf("\nEnter value of Sx & Sy: ");
    scanf("%f%f",&sx,&sy);
    
    printf("\nEnter Fixed Point Xf & Yf: ");
    scanf("%d%d",&xf,&yf);
    
    scale[0][0]=sx;
    scale[0][1]=0;
    scale[0][2]=xf * (1 - sx);
    
    scale[1][0]=0;
    scale[1][1]=sy;
    scale[1][2]=yf * (1 - sy);
    
    scale[2][0]=0;
    scale[2][1]=0;
    scale[2][2]=1;
    
    printf("\n");
    for(int i=0;i<3;i++)
    {
        for(int j=0;j<3;j++)
        {
            printf("\t%f",scale[i][j]);
        }
        printf("\n");
    }
    mul(input,scale,output,v);
        for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              printf("\t%f",output[i][j]);
          }
          printf("\n");
      }
    
}
void Reflection()
{
      for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              printf("\t%d",input[i][j]);
          }
          printf("\n");
      }
    
    for(int i=0;i<3;i++)
    {
        for(int j=0;j<3;j++)
        {
            if(i==j)
            {
              ref[i][j]=1;
            }
            else
            {
              ref[i][j]=0;
            }
        }
    }
    int ch=0;
     printf("\n1.Reflection about X-axis\n2.Reflection about Y-axis\n3.Reflection to an Axis Perpendicular to xy plane\n4.Reflection about Y=X\n5.Reflection about Y=-X\nEnter Choice: ");
     scanf("%d",&ch);
    switch(ch)
    {
        case 1:
        ref[1][1]=-1;
        break;
        
        case 2:
        ref[0][0]=-1;
        break;
        
        case 3:
         ref[1][1]=-1;
         ref[0][0]=-1;
        break;
        
        case 4:
        ref[0][0]=0;
        ref[1][1]=0;
        ref[0][1]=1;
        ref[1][0]=1;
        break;
        
        case 5:
        ref[0][0]=0;
        ref[1][1]=0;
        ref[0][1]=-1;
        ref[1][0]=-1;
        break;
    }
    
    printf("\n");
    for(int i=0;i<3;i++)
    {
        for(int j=0;j<3;j++)
        {
            printf("\t%f",ref[i][j]);
        }
        printf("\n");
    }
    printf("\n");
    //output matrix
    mul(input,ref,output,v);
    
    for(int i=0;i<3;i++)
    {
        for(int j=0;j<v;j++)
        {
            printf("\t%f",output[i][j]);
        }
        printf("\n");
    }
}

void ShearMatrix()
{   
      for(int i=0;i<3;i++)
      {
          for(int j=0;j<v;j++)
          {
              printf("\t%d",input[i][j]);
          }
          printf("\n");
      }
    int ch;
    printf("\n1.X shear\n2.Y shear\nEnter Choice: ");
    scanf("%d",&ch);
    float xref,yref,shx,shy;
    switch(ch)
    {
        case 1:
        printf("\nEnter Value of X Shear: ");
        scanf("%f",&shx);
         printf("\nEnter Value of Y Ref: ");
        scanf("%f",&yref);
        
        xshear[0][0]=1;
        xshear[0][1]=shx;
        xshear[0][2]=-shx*yref;
        
        xshear[1][0]=0;
        xshear[1][1]=1;
        xshear[1][2]=0;
        
        xshear[2][0]=0;
        xshear[2][1]=0;
        xshear[2][2]=1;
        
        mul(input,xshear,output,v);
        break;
        
        case 2:
        printf("\nEnter Value of Y Shear: ");
        scanf("%f",&shy);
        printf("\nEnter Value of X Ref: ");
        scanf("%f",&xref);
        
        yshear[0][0]=1;
        yshear[0][1]=0;
        yshear[0][2]=0;
        
        yshear[1][0]=shy;
        yshear[1][1]=1;
        yshear[1][2]=-shy*xref;
        
        yshear[2][0]=0;
        yshear[2][1]=0;
        yshear[2][2]=1;
         mul(input,yshear,output,v);
    }
    

}

void BresLine(int xa,int ya,int xb,int yb)
{
      int dx,dy,d;
      dx=xb-xa;
      dy=yb-ya;
      int c,r,f;
      int i=0;
      if(abs(dx)>abs(dy))
      {
          d=2*abs(dy)-abs(dx);
          if(dx>0)
          {
              c=xa;
              r=ya;
              f=xb;
          }
          else
          {
              c=xb;
              r=yb;
              f=xa;
          }
          
          while(f>c)
          {
              if(d<0)
              {
                  c=c+1;
                  d=d+2*abs(dy);
              }
              else
              {
                  if(dx>0 && dy>0 || dx<0 && dy<0)
                  {
                      r=r+1;
                  }
                  else
                  {
                      r=r-1;
                  }
                  c=c+1;
                  d=d+2*abs(dy)-2*abs(dx);
              }
                glVertex2d(c,r);
         }
      }
      else
      {
          d=2*abs(dx)-abs(dy);
          if(dy>0)
          {
              c=xa;
              r=ya;
              f=yb;
          }
          else
          {
              c=xb;
              r=yb;
              f=ya;
          }
          
          while(f>r)
          {
              if(d<0)
              {
                  r=r+1;
                  d=d+2*abs(dx);
              }
              else
              {
                  if(dx>0 && dy>0 || dx<0 && dy<0)
                  {
                      c=c+1;
                  }
                  else
                  {
                      c=c-1;
                  }
                  r=r+1;
                  d=d+2*abs(dx)-2*abs(dy);
              }  
               glVertex2d(c,r);
          }
          
      }
}


void Draw()
{
    glClear(GL_COLOR_BUFFER_BIT);
    glBegin(GL_POINTS);

    BresLine(-320,0,320,0);
    BresLine(0,-240,0,240);
    glEnd();
    glBegin(GL_POINTS);
    glColor3f(1,0,0);
    for(int i=0;i<v;i++)
    {    
        BresLine(input[0][i],input[1][i],input[0][(1+i)%v],input[1][(1+i)%v]);
    }
    glEnd();
   

    glBegin(GL_POINTS);     
    glColor3f(0,1,0);
    for(int i=0;i<v;i++)
    {
       BresLine(output[0][i],output[1][i],output[0][(1+i)%v],output[1][(1+i)%v]);
    }
    
    glEnd();
    glFlush();
}


int main(int argc,char **argv)
{

    printf("\nEnter no. of Vertices: ");
    scanf("%d",&v);
    
    
    for(int i=0; i<v;i++)
    {
        printf("\nX[%d]:",i+1);
        scanf("%d",&input[0][i]);
        printf("\nY[%d]:",i+1);
        scanf("%d",&input[1][i]);
        input[2][i]=1;
    }
        
     //Translation();   
     //Rotation();
     //Scaling();
     //Reflection();
     ShearMatrix();
        
    glutInit(&argc,argv);
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
    glutInitWindowPosition(0,0);
    glutInitWindowSize(640,480);
    glutCreateWindow("Window");
    glClearColor(1,1,1,0);
    glColor3f(0,0,0);
    gluOrtho2D(-320,320,-240,240);
    glutDisplayFunc(Draw);
    
    glutMainLoop();
    
    return 0;
}

// Slip10
#include <GL/glut.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

int xmin, xmax, ymin, ymax;

struct ver {
    float x, y;
    int outcode[4];
} p1, p2, orig1, orig2;

int Round(float a) {
    return (int)(a + 0.5);
}

void ddaline(int xa, int ya, int xb, int yb) {
    int dx = xb - xa;
    int dy = yb - ya;
    int steps = abs(dx) > abs(dy) ? abs(dx) : abs(dy);
    float xinc = dx / (float)steps;
    float yinc = dy / (float)steps;
    float x = xa, y = ya;

    for (int i = 0; i <= steps; i++) {
        glVertex2i(Round(x), Round(y));
        x += xinc;
        y += yinc;
    }
}

void calculate(struct ver *p) {
    for (int j = 0; j < 4; j++) p->outcode[j] = 0;

    if (p->x < xmin) p->outcode[3] = 1;  // Left
    if (p->x > xmax) p->outcode[2] = 1;  // Right
    if (p->y < ymin) p->outcode[1] = 1;  // Bottom
    if (p->y > ymax) p->outcode[0] = 1;  // Top

    for (int k = 0; k < 4; k++) printf("%d", p->outcode[k]);
}

void windowcliping() {
    calculate(&p1);
    printf("\n");
    calculate(&p2);

    orig1 = p1;
    orig2 = p2;

    int flag1 = 0;
    for (int i = 0; i < 4; i++) {
        if (p1.outcode[i] == 1 || p2.outcode[i] == 1) {
            flag1 = 1;
            break;
        }
    }

    if (flag1 == 0) {
        printf("\nFully Accepted");
    } else {
        int flag2 = 0;
        for (int j = 0; j < 4; j++) {
            if (p1.outcode[j] == 1 && p2.outcode[j] == 1) {
                flag2 = 1;
                break;
            }
        }

        if (flag2 == 1) {
            printf("\nRejected");
            return; // Don't draw
        } else {
            printf("\nPartially Accepted");
            float m = (orig2.y - orig1.y) / (orig2.x - orig1.x);
            printf(" | Slope: %f", m);

            if (p1.outcode[0]) { p1.y = ymax; p1.x = orig1.x + (ymax - orig1.y) / m; }
            if (p1.outcode[1]) { p1.y = ymin; p1.x = orig1.x + (ymin - orig1.y) / m; }
            if (p1.outcode[2]) { p1.x = xmax; p1.y = orig1.y + m * (xmax - orig1.x); }
            if (p1.outcode[3]) { p1.x = xmin; p1.y = orig1.y + m * (xmin - orig1.x); }

            if (p2.outcode[0]) { p2.y = ymax; p2.x = orig2.x + (ymax - orig2.y) / m; }
            if (p2.outcode[1]) { p2.y = ymin; p2.x = orig2.x + (ymin - orig2.y) / m; }
            if (p2.outcode[2]) { p2.x = xmax; p2.y = orig2.y + m * (xmax - orig2.x); }
            if (p2.outcode[3]) { p2.x = xmin; p2.y = orig2.y + m * (xmin - orig2.x); }
        }
    }

    printf("\nNew coordinates: (%.2f, %.2f) -> (%.2f, %.2f)", p1.x, p1.y, p2.x, p2.y);

    ddaline(Round(p1.x), Round(p1.y), Round(p2.x), Round(p2.y));
}

void Draw() {
    glClear(GL_COLOR_BUFFER_BIT);

    // Draw clipping window in blue
    glColor3f(0, 0, 1);
    glBegin(GL_POINTS);
    ddaline(xmin, ymin, xmax, ymin);
    ddaline(xmax, ymin, xmax, ymax);
    ddaline(xmax, ymax, xmin, ymax);
    ddaline(xmin, ymax, xmin, ymin);
    glEnd();

    // Draw original line in RED
    glColor3f(1, 0, 0);
    glBegin(GL_POINTS);
    ddaline(Round(p1.x), Round(p1.y), Round(p2.x), Round(p2.y));
    glEnd();

    // Wait a bit (optional)


    // Draw clipped line in BLACK
    glColor3f(0, 0, 0);
    glBegin(GL_POINTS);
    windowcliping();
    glEnd();

    glFlush();
}

int main(int argc, char **argv) {
    printf("Enter the window (xmin ymin xmax ymax): ");
    scanf("%d%d%d%d", &xmin, &ymin, &xmax, &ymax);

    printf("Enter the coordinates of the line (x1 y1 x2 y2): ");
    scanf("%f%f%f%f", &p1.x, &p1.y, &p2.x, &p2.y);

    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
    glutInitWindowPosition(0, 0);
    glutInitWindowSize(640, 480);
    glutCreateWindow("Cohen-Sutherland Window Clipping");
    glClearColor(1, 1, 1, 0);
    gluOrtho2D(0, 640, 0, 480);
    glutDisplayFunc(Draw);
    glutMainLoop();

    return 0;
}

//Slip11
#define GL_SILENCE_DEPRECATION
#include <GLUT/glut.h>
#include <stdlib.h>
#include <stdio.h>

int v; 
int input[2][10];
float bcol[3];
float fcol[3];
int x, y;

int xa, ya, xb, yb;
int ROUND(float a){
    return (int)(a+0.5);
}

void dda(int xa, int ya, int xb, int yb){
    int dx, dy, steps;
    dx = xb - xa;
    dy = yb - ya;

    if(abs(dx)>abs(dy)){
        steps = abs(dx);
    }
    else{
        steps = abs(dy);
    }

     float xinc, yinc;

    xinc = (float)dx/steps;
    yinc = (float)dy/steps;
    
    float x = xa;
    float y = ya;

    for(int i = 0; i<steps; i++){
        glBegin(GL_POINTS);
        x = x + xinc;
        y = y + yinc;
        glVertex2d(ROUND(x), ROUND(y));
    }
    glEnd();
}

void bfill(int x, int y, float bcol[3], float fcol[3]){
    float current[3];
    glReadPixels(x, y,1.0, 1.0, GL_RGB, GL_FLOAT, current);
    
    

    if((bcol[0] != current[0] || bcol[1] != current[1] || bcol[2] != current[2]) && 
    (fcol[0] != current[0] || fcol[1] != current[1] || fcol[2] != current[2]))
    {
        glColor3f(fcol[0], fcol[1], fcol[2]);
        glBegin(GL_POINTS);
        glVertex2d(x,y);
        glEnd();
        glFlush();

        bfill(x+1, y, bcol, fcol);
        bfill(x-1, y, bcol, fcol);
        bfill(x, y+1, bcol, fcol);
        bfill(x, y-1, bcol, fcol);

    }
}

void draw(){
    glClear(GL_COLOR_BUFFER_BIT);
    glColor3f(1,1,1);
    
    int i;
    for(i = 0; i < v-1; i++){
        dda(input[0][i],input[1][i],input[0][i+1], input[1][i+1]);
    }
    dda(input[0][i], input[1][i], input[0][0], input[1][0]);
    
    glEnd();
    glFlush();

    bfill(x, y, bcol, fcol);
}

int main(int argc, char** argv) {
    printf("Enter no of vertices: ");
    scanf("%d", &v);

    for(int i = 0; i < v; i++){
        printf("Enter x,y: ");
        scanf("%d %d", &input[0][i], &input[1][i]);
    }

    printf("\nBoundary color: ");
    for(int i =0 ; i<3; i++){
        scanf("%f", &bcol[i]);
    }

    printf("\nFill color: ");
    for(int i =0 ; i<3; i++){
        scanf("%f", &fcol[i]);
    }

    printf("Seed Points (x,y): ");
    scanf("%d %d", &x, &y);

    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
    glutInitWindowPosition(0,0);
    glutInitWindowSize(640,480);
    glutCreateWindow("DDA");
    glClearColor(0,0,0,0);
    
    gluOrtho2D(0,640,0,480);
    glutDisplayFunc(draw);
    glutMainLoop();
    return 0;
}

#define GL_SILENCE_DEPRECATION
#include<GLUT/glut.h>
#include<stdlib.h>
#include<stdio.h>

int input[2][10];
int v,x,y;
float bcol[3];
float backcol[3];
float fcol[3];

void FloodFill(int x,int y,float backcol[3],float fcol[3])
{
	float current[3];
	glReadPixels(x,y,1.0,1.0,GL_RGB,GL_FLOAT,current);
	
	if((current[0]==backcol[0] && current[1]==backcol[1] && current[2]==backcol[2] ) && (current[0]!=fcol[0] || current[1]!=fcol[1] || current[2]!=fcol[2] ) )
	{
		glColor3f(fcol[0],fcol[1],fcol[2]);
		glBegin(GL_POINTS);
		glVertex2d(x,y);
		glEnd();
		glFlush();
		
		FloodFill(x+1,y,backcol,fcol);
		FloodFill(x-1,y,backcol,fcol);
		FloodFill(x,y+1,backcol,fcol);
		FloodFill(x,y-1,backcol,fcol);
	}
}

int ROUND(float a)
{
	return (int) (a+0.5);
}
void DDA(int xa, int ya,int xb, int yb)
{
	int dx,dy,steps;
	dx=xb-xa;
	dy=yb-ya;
	
	if(abs(dx)>abs(dy))
	{
		steps=abs(dx);
	}
	else
	{
		steps=abs(dy);
	}
	
	float xinc,yinc;
	xinc=(float)dx/steps;
	yinc=(float)dy/steps;
	
	glVertex2d(xa,ya);
	
	float x=xa,y=ya;
	for (int i=0;i<steps;i++)
	{
		x=x+xinc;
		y=y+yinc;
		glVertex2d(ROUND(x),ROUND(y));
	}
}

void Draw()
{
	glClear(GL_COLOR_BUFFER_BIT);
	glBegin(GL_POINTS);
	int i;
	for (i=0;i<v-1;i++)
	{	
		
		
		DDA(input[0][i],input[1][i],input[0][i+1],input[1][i+1]);
	}
	
		DDA(input[0][i],input[1][i],input[0][0],input[1][0]);
		
	glEnd();
	glFlush();
	
	FloodFill( x,y,backcol,fcol);
	
}

int main(int argc,char **argv)
{
	printf("\nEnter no of Vertices: ");
	scanf("%d",&v);
	
	for(int i=0;i<v;i++)
	{
		printf("\nX[%d]: ",i+1);
		scanf("%d",&input[0][i]);
		printf("\nY[%d]: ",i+1);
		scanf("%d",&input[1][i]);
	}
	
	printf("\nEnter Inside Point: ");
	scanf("%d%d",&x,&y);
	
	for(int i=0;i<3;i++)
	{
		printf("\nBackgrount Color: ");
		scanf("%f",&backcol[i]);
	}
	
	for(int i=0;i<3;i++)
	{
		printf("\nFill Color: ");
		scanf("%f",&fcol[i]);
	}
	

	glutInit(&argc,argv);
	glutInitDisplayMode(GLUT_SINGLE || GLUT_RGB);
	glutInitWindowPosition(0,0);
	glutInitWindowSize(640,480);
	glutCreateWindow("Flood Filling");
	glClearColor(backcol[0],backcol[1],backcol[2],0);
	glColor3f(0,0,0);
	gluOrtho2D(0,640,0,480);
	glutDisplayFunc(Draw);
	glutMainLoop();
	
	return 0;
}

// Slip13

	Bline(200,50,220,50);
	Bline(220,50,220,150);
	Bline(220,150,200,150);
	Bline(200,150,200,50);
	
	Bline(260,50,280,50);
	Bline(280,50,280,150);
	Bline(280,150,260,150);
	Bline(260,150,260,50);
	
	Bline(200,150,280,150);
	Bline(280,150,280,280);
	Bline(280,280,200,280);
	Bline(200,280,200,150);
	
	Bline(235,280,235,290);
	Bline(245,280,245,290);
	
	Bcircle(240,320,30);
	
	Bline(200,280,150,220);
	Bline(150,220,200,260);
	
	Bline(280,280,330,220);
	Bline(330,220,280,260);
	
	Bcircle(225,325,5);
	Bcircle(255,325,5);
	
	Bline(240,320,235,315);
	Bline(235,315,245,315);
	Bline(245,315,240,320);
	
	Bline(230,310,230,305);
	Bline(230,305,250,305);
	Bline(250,305,250,310);
	Bline(250,310,230,310);

// Slip14
#include<stdio.h>
#include<GL/glut.h>
#include<math.h>

int xa, ya, xb, yb, xc, yc, r;

int ROUND(float a){
  return (int) (a + 0.5);
}

void BresCircle(int xc, int yc, int r){
  int x, y, d;
  x = 0; y = r; d = 3-2*r;

  do{
  glVertex2d(xc+x,yc+y);
  glVertex2d(xc+y,yc+x);
  glVertex2d(xc-x,yc-y);
  glVertex2d(xc-y,yc-x);
  glVertex2d(xc+x,yc-y);
  glVertex2d(xc-x,yc+y);
  glVertex2d(xc-y,yc+x);
  glVertex2d(xc+y,yc-x);

    if(d<0){
      x = x+1;
      d = d+4*x+6;
    }

    else{
      x = x+1;
      y = y-1;
      d = d+4*x-4*y+10;
    }
  }
  while(y>=x);
}

void shape(){
    BresCircle(xc,yc,r);
    BresCircle(xc+2*r,yc,r);
    BresCircle(xc+r,yc+r,r);
    BresCircle(xc+r,yc-r,r);

}

void Mouse(int button, int state, int x, int y){
    if (button == GLUT_LEFT_BUTTON && state == GLUT_DOWN){
            xc = x;
            yc = 720-y;
            glutPostRedisplay();
       }
}

void Draw(){
glClear(GL_COLOR_BUFFER_BIT);
glBegin(GL_POINTS);
shape();
glEnd();
glFlush();
}

int main(int argc ,char **argv){
printf("\nEnter radius length: ");
scanf("%d", &r);

glutInit(&argc, argv);
glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
glutInitWindowPosition(0,0);
glutInitWindowSize(1080,720);
glutCreateWindow("WINDOW");
glClearColor(1,1,1,0);
glColor3f(0,0,0);
gluOrtho2D(0,1080,0,720);
glutDisplayFunc(Draw);
glutMouseFunc(Mouse);
glutMainLoop();
return 0;
}

// Slip15
#include<stdio.h>
#include<GL/glut.h>
#include<math.h>

int xa, ya, xb, yb, xc, yc, r;

int ROUND(float a){
  return (int) (a + 0.5);
}

void BresCircle(int xc, int yc, int r){
  int x, y, d;
  x = 0; y = r; d = 3-2*r;

  do{
  glVertex2d(xc+x,yc+y);
  glVertex2d(xc+y,yc+x);
  glVertex2d(xc-x,yc-y);
  glVertex2d(xc-y,yc-x);
  glVertex2d(xc+x,yc-y);
  glVertex2d(xc-x,yc+y);
  glVertex2d(xc-y,yc+x);
  glVertex2d(xc+y,yc-x);

    if(d<0){
      x = x+1;
      d = d+4*x+6;
    }

    else{
      x = x+1;
      y = y-1;
      d = d+4*x-4*y+10;
    }
  }
  while(y>=x);
}

void shape(){
    BresCircle(xc,yc,r);
    BresCircle(xc+2*r,yc,r);
    BresCircle(xc-2*r,yc,r);
    BresCircle(xc+2*r*(0.5),yc+2*r*(0.866),r);
    BresCircle(xc+2*r*(-0.5),yc+2*r*(0.866),r);
    BresCircle(xc+2*r*(-0.5),yc+2*r*(-0.866),r);
    BresCircle(xc+2*r*(0.5),yc+2*r*(-0.866),r);
    BresCircle(xc,yc,2*r);
}

void Mouse(int button, int state, int x, int y){
    if (button == GLUT_LEFT_BUTTON && state == GLUT_DOWN){
            xc = x;
            yc = 720-y;
            glutPostRedisplay();
       }
}

void Draw(){
glClear(GL_COLOR_BUFFER_BIT);
glBegin(GL_POINTS);
shape();
glEnd();
glFlush();
}

int main(int argc ,char **argv){

printf("\nEnter radius length: ");
scanf("%d", &r);

glutInit(&argc, argv);
glutInitDisplayMode(GLUT_SINGLE | GLUT_RGB);
glutInitWindowPosition(0,0);
glutInitWindowSize(1080,720);
glutCreateWindow("WINDOW");
glClearColor(1,1,1,0);
glColor3f(0,0,0);
gluOrtho2D(0,1080,0,720);
glutMouseFunc(Mouse);
glutDisplayFunc(Draw);
glutMainLoop();
return 0;
}

// Slip16