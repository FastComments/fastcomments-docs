FastComments Access Control works by assigning `Pages` and `Users` into the desired groups.

A group is simply a string identifier, like `GREEN` or `abc-123`.

`Users` and `Pages` are not just limited to one group, instead are limited to `100` and `1000` groups, respectively. 

#### Accessing Unauthorized Pages

If a user tries to access a page they don't have access to, they will see an error message, like below:

<div class="screenshot white-bg">
    <div class="title">Authorization Failure Example</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Authorization Failure Example" />
</div>

The message text can be customized.
