FastComments Access Control works by assigning `Pages` and `Users` to the desired groups.

A group is simply a string identifier, like `GREEN` or `abc-123`.

`Users` and `Pages` are not limited to a single group. They are limited to `100` and `1000` groups, respectively. 

#### Accessing Unauthorized Pages

If a user tries to access a page they don't have access to, they'll see an error message like the one below:

<div class="screenshot white-bg">
    <div class="title">Authorization Failure Example</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Authorization Failure Example" />
</div>

The message text can be customized.