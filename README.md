# What do in need to do in this code base?

All the below shounld be in its own database file using the turso stuff

Some crud stuff

- Make it possible to create a horse stable
- Make it possible to add a horse to the stable
- Make it possible to remove a horse from the stable
- Make it possible to list all the horses in the stable

Optional shit generated by supermaven

- Make it possible to list all the horses in the stable that are not yet sold
- Make it possible to list all the horses in the stable that are sold
- Make it possible to list all the horses in the stable that are sold and not yet returned
- Make it possible to list all the horses in the stable that are returned
- Make it possible to list all the horses in the stable that are returned and not yet sold

For user authentication

I guess we need an admin user only and ablility to register workers

- admin user created by us, some script that creates the user
- worker user created by the admin in the dashboard

For our nice horses

- Clean the horse ( sets is being cleaned to true )
- Feed the horse ( sets is being fed to true )
- Train the horse ( sets is being trained to true )

the above should have a timer, and autmatically when the timer is up, the horse status is set to false

Create the dbs in a destination folder able to be specified by the installer


## What i wrote down for myself
- See all barns
- Who is working on what barn
-- Current horse in a barn
- History info of a barn , last time it was cleaned, last person who worked on it.
- See all the horses with info about them
- What is the horse doing right now, search for it in a list with like autocomplete based on any attribute
- History of a horse with some statistics
- Employee info
- Be ably to assign a horse to an employee
- Allow owners to access the data and see what their horse is doing.


## TODOS
- [] Hash passwords i know but i do not care for now, maybe use other auth way than password
- [] Add stronghold for strong session when logging in
- [] Generate test data 
- [] Can i link my try_from to the table creation code ? i do not want to use an orm, diy
