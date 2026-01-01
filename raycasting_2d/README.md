# Raycasting 2D

This repository is all about graphics, I called it "rust 3D" because originally I wanted to make some 3D graphics in
rust with the idea of learning rust, but I need to do more than just one thing, and I believe raycasting is a very nice
concept to learn about graphics programming and it's also fun to implement.

## What is Raycasting?

Raycasting is a technique to cast rays from a point (usually the camera) into a 2D or 3D space to determine what objects
are visible from that point. In 2D raycasting, rays are cast in a 2D plane to determine which walls or objects are
visible from the camera's position. This technique is often used in early 3D games to create a pseudo-3D effect by
rendering 2D sprites based on the distance from the camera. (I want to do that later in a project I called
`monchestein`. That word combines `moncho` and `wolfestein` because I want to make a game like wolfenstein 3D but I
don't want to implement everything, and `moncho` in Argentinan slang means "to do something without much effort".)

## How does it work?
The basic idea of raycasting is to cast rays from the camera's position in a specific direction and check for
intersections with objects in the scene.
