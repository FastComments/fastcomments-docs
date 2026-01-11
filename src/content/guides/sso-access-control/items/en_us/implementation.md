#### Mentioning Users in Other Groups

If two users belong to two different sets of groups, and there is no intersection, they will not be able to `@mention` each other.

If a user manually types an `@mention` and submits their comment, it will remain as plain text. The other user will not be tagged.

#### Maintaining the Groups

`Groups` are defined using the `Pages` and `SSOUsers` API resources, respectively.

The `Pages` API can be invoked to define the set of groups allowed to access the page. By default, all groups, and users that do
not belong to a group, have access.

Similarly, the `SSOUsers` API can be invoked to define the groups associated with each user.

For both resources, there are no restrictions on when the groups can be set or updated.

If the only goal is to limit users from `@mention`'ing each other, then `Pages` do not have to be taken into consideration.

##### Note!

Defining and updating the SSO user groups doesn't require using the API, and can instead be updated automatically by defining the
group IDs in the SSO payload passed to the comment widget. However, for large lists of groups, this is not recommended as the user
would have to submit this payload on every page load.