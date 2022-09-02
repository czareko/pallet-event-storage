# Pallet EventStorage

Simple event storage with a parametrized history size

The idea is to store custom events in our pallet, access to this functionality should be limited only to one predefined account. All events older than a configured period of time should be removed automatically.

This example is a tech representation of the idea because we didn’t focus on how to use these events. The first step to change it would be to redesign the CustomEvent struct

https://github.com/czareko/pallet-event-storage/blob/d83af4945d7516a3c20c943d6cca8e0ca680ec83/src/lib.rs#L20-L27

### Parametrization

|Parameter| Description                    |
|------|--------------------------------|
|HistorySize: i64| History size defined in seconds|
|AuthorizedAccountId: AccountId| Basic account id privilaged to create custome events |

Here we have a sample from [mock.rs](https://github.com/czareko/pallet-event-storage/blob/feat/documentation/src/mock.rs)

https://github.com/czareko/pallet-event-storage/blob/d83af4945d7516a3c20c943d6cca8e0ca680ec83/src/mock.rs#L29-L32


### How to test it?

The best description for the whole functionalities we will find in [tests.rs](https://github.com/czareko/pallet-event-storage/blob/feat/documentation/src/tests.rs)

``
cargo test
``

### What we could do better?

#### 1. Caller verification

Now we have a very basic idea for this, but not as simple as using ``root`` as a caller. There are many good examples how to prepare this in the more advanced way. For example: pallet_sudo, pallet_membership, pallet_identity.


#### 2. Storage

There are many ways how we could protect our storage better. For example, we could set a maximum number of items and a single item size.
Access to the Storage should be limited as much as possible.

Frame V2 has good support for running the pallet in more than one instance, I don't see a business example of why we would like to do this with this pallet, but of course, if we have a serious need we could think about it.

#### 3. Public methods

We have a few public methods in our pallet. We use them only for tests. The question is if we need them, or maybe it would be possible to organize this pallet in a different way to hide everything that is possible.

#### 4. CustomEvent - structure

Now it's a completely tech representation. With more detailed business requirements, more needs can come. This could change our idea for the map key in our Storage and how we search history items.

#### 5. CustomEvents vs Pallet Events

Now we generate simple system events after every method execution, but we don't know the external/integration requirements for this pallet, so in a serious example we should think about this part more.

#### 6. Errors

Now Exception handling show that we know how to use them, but we should try to catch more negative behaviors.
