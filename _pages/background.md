---
# YAML frontmatter
layout: default
title: Background with Object
# Removed incompatible asset declarations
description: Use JavaScript to have an in motion background.
permalink: /game-background
---

<canvas id="world"></canvas>

<script>
    // Declarations: scene, assets
    const canvas = document.getElementById("world");
    const ctx = canvas.getContext('2d');
    const backgroundImg = new Image();
    const spriteImg = new Image();
    backgroundImg.src = '/mataiodoxion_2026/assets/images/stars.jpg';
    spriteImg.src = '/mataiodoxion_2026/assets/images/mataiodoxia.png';

    // Load images
    let imagesLoaded = 0;
    backgroundImg.onload = function() {
        imagesLoaded++;
        startGameWorld();
    };
    spriteImg.onload = function() {
        imagesLoaded++;
        startGameWorld();
    };

    // Init sequence
    function startGameWorld() {
        // Wait until images load
        // Should just use async...
        if (imagesLoaded < 2) return;

        // Base object
        class GameObject {
            // Construct an Object
            constructor(image, width, height, x = 0, y = 0, speedRatio = 0) {
                this.image = image;
                this.width = width;
                this.height = height;
                this.x = x;
                this.y = y;
                this.speedRatio = speedRatio;
                this.speed = GameWorld.gameSpeed * this.speedRatio;
            }
            
            update() {}

            // Actually draw the object
            draw(ctx) {
                ctx.drawImage(this.image, this.x, this.y, this.width, this.height);
            }
        }

        // Background is a fork of GameObject
        class Background extends GameObject {
            // Construct a Background object
            constructor(image, gameWorld) {
                // Fill entire canvas
                super(image, gameWorld.width, gameWorld.height, 0, 0, 0.1);
            }

            // Update method
            update() {
                this.x = (this.x - this.speed) % this.width;
            }

            // Draw the actual background
            draw(ctx) {
                ctx.drawImage(this.image, this.x, this.y, this.width, this.height);
                ctx.drawImage(this.image, this.x + this.width, this.y, this.width, this.height);
            }
        }

        // Player is a fork of GameObject
        class Player extends GameObject {
            // Construct a Player object
            constructor(image, gameWorld) {
                const width = image.naturalWidth / 2;
                const height = image.naturalHeight / 2;
                const x = (gameWorld.width - width) / 2;
                const y = (gameWorld.height - height) / 2;
                super(image, width, height, x, y);
                this.baseY = y;
                this.frame = 0;
            }

            // Update pos per frame
            update() {
                this.y = this.baseY + Math.sin(this.frame * 0.05) * 20;
                this.frame++;
            }
        }

        // Master game object
        class GameWorld {
            static gameSpeed = 5;

            // Construct the scene
            constructor(backgroundImg, spriteImg) {
                this.canvas = document.getElementById("world");
                this.ctx = this.canvas.getContext('2d');
                this.width = window.innerWidth;
                this.height = window.innerHeight;
                this.canvas.width = this.width;
                this.canvas.height = this.height;
                this.canvas.style.width = `${this.width}px`;
                this.canvas.style.height = `${this.height}px`;
                this.canvas.style.position = 'absolute';
                this.canvas.style.left = `0px`;
                this.canvas.style.top = `${(window.innerHeight - this.height) / 2}px`;

                // Append Backround and Player objects
                this.objects = [
                    new Background(backgroundImg, this),
                    new Player(spriteImg, this)
                ];
            }

            // Keep game alive
            gameLoop() {
                this.ctx.clearRect(0, 0, this.width, this.height);
                for (const obj of this.objects) {
                obj.update();
                obj.draw(this.ctx);
                }
                requestAnimationFrame(this.gameLoop.bind(this));
            }
            
            // Start the game
            start() {
                this.gameLoop();
            }
        }

        // Actually build and run the "game"
        const world = new GameWorld(backgroundImg, spriteImg);
        world.start();
    }
</script>