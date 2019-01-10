# Migrate your software.

Saying goodbye to a reliable old framework can be scary.
You need to make sure that every piece of software has been migrated and
working correctly when it gets in production.
With Ambassade it is easy to set up dependencies and pieces of your code in a
hybrid way, making your migrations smoother.
Once you have migrated a part of your code, Ambassade checks for remaining
dependencies in your configuration before you remove your old code.

# Control over heterogenous packages.

In some cases, you want to make use of libraries/packages that take some effort
to be compatible with your project. Take the OPC-UA protocol for example. The
protocol has limited implementations because of a 400+ pages counting
specification. There are some useful C libraries available that support the
OPC-UA protocol. If you want to combine the library with Node.js, you have some
solutions, for example:

* Create an NPM package that interfaces with the C implementation using Node's
Native API.

* Seperate your project in a server-side part, using [Oat++](https://oatpp.io)
to create an executable for the server, containing the back-end code. The
client-side part will consist of front-end code.

* Compile the library to WebAssembly and "glue" the compiled binary to your
Node.js code.

Without having a central package manager that organizes all the different parts
of the code in different languages, keeping control over your software will be
difficult. There are some package managers and git features (like submodules
and mergetrees) that can help you with managing your packages. However, most
of them will not provide a satisfying solution. Ambassade is dedicated to manage
heterogenous packages, and does not even require special configuration files in
the dependencies themselves.

# Make your software more modular.

Because Ambassade takes care of dependencies, it gives you as developer more
freedom to make your projects more modular. You could split your project in
more repositories. The advantage of modulating your project is that you can
re-use code more easily. If a deprecated feature should not be implemented
in your project anymore, you can rebuild your project without having to delete
files. Maybe that code could be useful later for future projects.

If you already have certain features implemented in project A, you can import
that feature in project B with one command, so you only have to focus on
calling the feature from your project.

# Keep the structure of non-trivial projects simple.

Once a successful project grows, and grows, it's structure will be less obvious.
The developers that developed the project from the start, know exactly the
relationships between dependencies. New developers that join the development
team will have a harder time figuring the project's structure out. An
easy-to-read configuration file and a flat dependency structure would be a
solution.

## What is a 'flat dependency structure'?

NPM version 2 uses a nested dependency structure. This basically means that a
dependency resides in a dependency, which resides in another dependency and so
on.

A flat dependency structure has all dependencies at the same level. Ambassade
puts all dependencies in one directory, the `dep` directory. This makes finding
dependencies easier and keeps directory paths relatively small.

# Backup all of your code with one command.

Having to checkout and backup all of your code manually, could take a lot of
time and joy. Automating checkouts and backups of your code takes a lot of time
as well and isn't that enjoyable. Initializing a Ambassade project with all your
code as dependencies saves a lot of time. This way you can use Ambassade as a
system to backup all important files.
