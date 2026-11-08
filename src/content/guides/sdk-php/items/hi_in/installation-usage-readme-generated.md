### आवश्यकताएँ

PHP 7.4 और बाद के संस्करण।
PHP 8.0 के साथ भी काम करना चाहिए।

### Composer

[Composer](https://getcomposer.org/) के माध्यम से बाइंडिंग्स इंस्टॉल करने के लिए, `composer.json` में निम्नलिखित जोड़ें:

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

फिर `composer install` चलाएँ

### मैनुअल इंस्टॉलेशन

फ़ाइलें डाउनलोड करें और `autoload.php` शामिल करें:

```php
<?php
require_once('/path/to/fastcomments/client/vendor/autoload.php');
```