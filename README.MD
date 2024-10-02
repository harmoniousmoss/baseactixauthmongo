# bActix Web + MongoDB Authentication Example

This repository is an example of an endpoint built with Actix Web and MongoDB, demonstrating user authentication and admin seeding. The project includes JWT authentication middleware, MongoDB connection handling, and routes for user signup and signin.

## Features:
- MongoDB Integration
- JWT Authentication Middleware
- Admin Seeding
- User Authentication
- Customizable Routes: Easily configurable routes for various HTTP methods.
- Environment Configuration: Use .env to configure MongoDB URI and port settings.

## Routes:
- GET /: Greet endpoint to confirm the server is running.
- POST /signup: Create a new user.
- POST /signin: Authenticate a user and return a JWT token.
