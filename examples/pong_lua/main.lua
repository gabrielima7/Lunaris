-- Example: Simple Pong game in Lua
-- Demonstrates how to create a game using the Lunaris Engine with Lua scripting.

-- Game state
local game = {
    left_paddle = { x = 50, y = 360, width = 20, height = 100, speed = 400 },
    right_paddle = { x = 1230, y = 360, width = 20, height = 100, speed = 400 },
    ball = { x = 640, y = 360, vx = 300, vy = 200, radius = 10 },
    left_score = 0,
    right_score = 0,
    screen_width = 1280,
    screen_height = 720
}

-- Helper function to check rectangle collision
local function rects_overlap(ax, ay, aw, ah, bx, by, bw, bh)
    return ax < bx + bw and ax + aw > bx and ay < by + bh and ay + ah > by
end

-- Reset ball to center
local function reset_ball()
    game.ball.x = game.screen_width / 2
    game.ball.y = game.screen_height / 2
    -- Reverse direction
    game.ball.vx = game.ball.vx > 0 and -300 or 300
    game.ball.vy = 200
end

-- Called once when the game starts
function on_init()
    print("Pong game initialized!")
    print("Controls:")
    print("  Left paddle: W/S")
    print("  Right paddle: Up/Down arrows")
end

-- Called every frame
function on_update(dt)
    -- Left paddle input (W/S)
    if lunaris.input.is_key_down("w") then
        game.left_paddle.y = game.left_paddle.y - game.left_paddle.speed * dt
    end
    if lunaris.input.is_key_down("s") then
        game.left_paddle.y = game.left_paddle.y + game.left_paddle.speed * dt
    end
    
    -- Right paddle input (arrows)
    if lunaris.input.is_key_down("up") then
        game.right_paddle.y = game.right_paddle.y - game.right_paddle.speed * dt
    end
    if lunaris.input.is_key_down("down") then
        game.right_paddle.y = game.right_paddle.y + game.right_paddle.speed * dt
    end
    
    -- Clamp paddles to screen
    local half_height = game.left_paddle.height / 2
    game.left_paddle.y = lunaris.math.clamp(
        game.left_paddle.y,
        half_height,
        game.screen_height - half_height
    )
    game.right_paddle.y = lunaris.math.clamp(
        game.right_paddle.y,
        half_height,
        game.screen_height - half_height
    )
    
    -- Move ball
    game.ball.x = game.ball.x + game.ball.vx * dt
    game.ball.y = game.ball.y + game.ball.vy * dt
    
    -- Ball collision with top/bottom walls
    if game.ball.y <= game.ball.radius or game.ball.y >= game.screen_height - game.ball.radius then
        game.ball.vy = -game.ball.vy
        lunaris.audio.play("bounce")
    end
    
    -- Ball collision with left paddle
    local ball_left = game.ball.x - game.ball.radius
    local ball_top = game.ball.y - game.ball.radius
    local ball_size = game.ball.radius * 2
    
    local lp = game.left_paddle
    local paddle_left = lp.x - lp.width / 2
    local paddle_top = lp.y - lp.height / 2
    
    if rects_overlap(ball_left, ball_top, ball_size, ball_size,
                     paddle_left, paddle_top, lp.width, lp.height) then
        if game.ball.vx < 0 then
            game.ball.vx = -game.ball.vx * 1.1  -- Speed up
            game.ball.x = lp.x + lp.width / 2 + game.ball.radius
            lunaris.audio.play("hit")
        end
    end
    
    -- Ball collision with right paddle
    local rp = game.right_paddle
    paddle_left = rp.x - rp.width / 2
    paddle_top = rp.y - rp.height / 2
    
    if rects_overlap(ball_left, ball_top, ball_size, ball_size,
                     paddle_left, paddle_top, rp.width, rp.height) then
        if game.ball.vx > 0 then
            game.ball.vx = -game.ball.vx * 1.1
            game.ball.x = rp.x - rp.width / 2 - game.ball.radius
            lunaris.audio.play("hit")
        end
    end
    
    -- Scoring
    if game.ball.x <= 0 then
        game.right_score = game.right_score + 1
        print("Score: " .. game.left_score .. " - " .. game.right_score)
        lunaris.audio.play("score")
        reset_ball()
    end
    
    if game.ball.x >= game.screen_width then
        game.left_score = game.left_score + 1
        print("Score: " .. game.left_score .. " - " .. game.right_score)
        lunaris.audio.play("score")
        reset_ball()
    end
end

-- Called every frame to render (would use actual draw calls)
function on_render()
    -- In a real implementation:
    -- lunaris.draw.clear(0, 0, 0)
    -- lunaris.draw.rect(game.left_paddle.x, game.left_paddle.y, ...)
    -- lunaris.draw.circle(game.ball.x, game.ball.y, game.ball.radius)
    -- lunaris.draw.text(game.left_score .. " - " .. game.right_score, ...)
end

-- Return the game table for the engine to use
return game
