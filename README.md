# Fe2O3: A finite element library

Fe2O3 is a generic scientific computation library focusing on finite elements written in [rust](https://www.rust-lang.org/).

## History

The project started out around a beer between three buddies looing to improve their coding skills. Let's see how it goes!

## Building

Being a rust project, Fe2O3 uses the [cargo](https://doc.rust-lang.org/cargo/) framework for building and testing.

## Contributing

### Development Workflow

For now, please perform changes in a separate branch (hopefully well named to reflect the changes it provides) and propose a merge request for review before merging into `master`.

### Formating

Let's try and follow the rust standard naming conventions defined [here](https://rust-lang.github.io/api-guidelines/naming.html).

Be sure to run [rustfmt](https://github.com/rust-lang/rustfmt) on your code before proposing it up for review and adding it into the repository (right now all the formating is just the default rust formating).

### Testing

Every new development should have associated unit testing. Developments that do not have unit tests will most likely not pass review.

Integration testing should be implemented when deemed fit.
