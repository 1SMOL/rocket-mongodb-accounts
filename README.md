# Rocket &heartsuit; MongoDB User Authentication

This is a learning project, and half decent starting point for newbies like me that would enjoy a reference on how to implement mongodb into their own projects for user authentication.

## Side notes

Don't forget to create your `Rocket.toml` in the crate root with your database pool; here's a reference to rocket's db pool crate documents--[rocket_db_pools](https://api.rocket.rs/v0.5-rc/rocket_db_pools/index.html)--if you don't set this up, rocket will panic. Since this is a MongoDB specific implementation, be sure to create your account at [MongoDB](https://mongodb.com) and create a 'shared' cluster to be used as your database for free. Please also read over the [MongoDB Rust driver docs](https://docs.rs/mongodb/latest/mongodb/) on how to interact with mongodb.

I will avoid any form of formatting that-to me-causes the code to be less readable compared to other languages that newbies like me come from.

If you do plan on making anything production ready; you'll obviously want to hash passwords with SHA-256 or other industry standard hashing algorithms for user account safety, you'll also need to do _a lot_ more sanitizing of user inputs, this can be done with JS or through the server itself.
