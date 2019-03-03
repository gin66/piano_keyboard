# Piano_Keyboard

[![Build Status](https://travis-ci.org/gin66/piano_keyboard.svg?branch=master)](https://travis-ci.org/gin66/piano_keyboard)

This crate provides the graphical elements in order to draw a piano keyboard
with close to realistic, pixel accurate appearance.

Reference for the dimension is this internet image:
![octave drawing](http://www.rwgiangiulio.com/construction/manual/layout.jpg)

The dimensions described have been used to create the elements of
a piano keyboard like for an octave like this:
![img](keyboard.png)

It is visible, that between white keys and even between white and black keys a gap
is ensured.

The graphical representation only provides the white and black areas for the keys.
Those areas are represented by pixel accurate, non-overlapping rectangles.
No aliasing or similar is done on this level.

Pixel accurate has the consequence, that in order to fill the requested width,
any gaps, white or black keys may need to be modified by up to one pixel.
Those changes may or may not be visible. If no adjustments have been made for
a given width and key range is reported by the function is_perfect()

If the enlargement of various elements does not succeed, then as last resort
technique the outter gaps are enlarged.

The gap between white and black keys can be removed by an option of the KeyboardBuilder.

The interface is prepared to be compatible for an extension towards a 3d keyboard.
That's why the returned keyboard is called Keyboard2D and the related build function
is called build2d().
