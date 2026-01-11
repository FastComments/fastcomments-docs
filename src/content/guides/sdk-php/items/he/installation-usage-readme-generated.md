### דרישות

PHP 7.4 ומעלה.
אמור גם לעבוד עם PHP 8.0.

### Composer

כדי להתקין את הספרייה דרך [Composer](https://getcomposer.org/), הוסף את השורות הבאות ל־`composer.json`:

```json
{
  "repositories": [
    {
      "type": "vcs",
      "url": "https://github.com/fastcomments/fastcomments-php.git"
    }
  ],
  "require": {
    "fastcomments/fastcomments-php": "*@dev"
  }
}
```

לאחר מכן הרץ `composer install`

### התקנה ידנית

הורד את הקבצים והכלל את `autoload.php`:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```