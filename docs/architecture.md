# Architecture

From version `2.0.0`, `gitbox` is adopting the [clean architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html), using the 4 layers described by Martin.

## Domain
Entities in this layer should be plain old objects, so just structs with factories (which checks for invariants) and getters.

Structs should contain owned types, while getter methods shall return references (`&T`, `Box<T>`, `Rc<T>`, ...; the specific type is not that important for the first iteration of an entity).

Entities may derive all the traits they need, but they should only derive used traits (so avoid blanket derives).

## Usecase
This layer contains 4 types of class: repositories, errors, configurations and usecases.

Repositories are the interfaces (`trait`s) for the inversion of dependencies. The name of repositories is composed by the name of the resource handled, the direction of the data (`Ingress` for "get" repositories, `Egress` for "send" repositories) and `Repository`; for example, the repository responsible for retrieving semantic versions will be named `SemanticVersionIngressRepository`.

Errors are self-explanatory.

Configurations are containers used to manipulate the behaviour of usecases.

Usecases contains the business rules of an operation, i.e. they define how the inputs, the outputs and the configuration are linked together. Each usecase should define one (and only one) atomic operation (for example, describing a new version and creating a tag, even if they are used the same command `describe`, are two separate usecases). Usecases must implement the trait `Usecase<T>`, which defines the method `execute`, it returns a `Result<T, AnyError>` (note: `AnyError` is just an alias for `Box<dyn Error>`).

## Application
This layer contains 6 types of class: options, errors, managers, presenters, repository\_impl, controllers.

Options are simple structs which contain options used by controllers. Options contain the configuration of all the usecases used by the controller, and other possible values which modify the controller behaviour.

Errors are self-explanatory.

Managers are the interfaces for the inversion of dependencies with the outer layer. Managers follow the same naming schema as repositories (but with `Manager` rather than `Repository`). Managers should retrieve or send objects composed by basic types (integers, strings, ...).

Presenters are modules which contain the implementation of traits like (but not exclusively) `Display` or `FromStr`, i.e. any trait that allows to transform basic types into domain entities and vice versa.

RepositoryImpl contains the implementation of repository traits. This implementations should require a manager and they should use presenters to transform the data into the needed type. For example an ingress repository impl should contain the corresponding manager which is used to retrieve the data, it should transform it (using a function in the specific presenter module) into domain entities, and it should return it to the caller (which is likely to be a usecase).

Controllers configure and start usecases (and the required repositories), based on Option objects. Usually, they need managers to initialize repositories. There is no specific rule about the usecases a controller should manage, but generally the usecases should be linked by a output-input relation (i.e. the output of one is the input of the others).

## Infrastructure
In this layer there are the outermost types of class: errors, helpers, interfaces and subcommands.

Errors are self-explanatory.


