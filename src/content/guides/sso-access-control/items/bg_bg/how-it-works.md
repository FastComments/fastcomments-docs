FastComments Access Control работи чрез присвояване на `Pages` и `Users` към желаните групи.

A group is simply a string identifier, like `GREEN` or `abc-123`.

`Users` and `Pages` are not just limited to one group. They are limited to `100` and `1000` groups, respectively. 

#### Достъп до неоторизирани страници

If a user tries to access a page they don't have access to, they will see an error message, like below:

<div class="screenshot white-bg">
    <div class="title">Authorization Failure Example</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Authorization Failure Example" />
</div>

Текстът на съобщението може да бъде персонализиран.

---