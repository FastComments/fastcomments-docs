The FastComments Drupal module replaces Drupal's built-in comments with a fast, real-time commenting system. The module is [published on drupal.org](https://www.drupal.org/project/fcom) and works with Drupal 10 and 11.

There are two ways to install it.

## Install with Composer

```
composer require drupal/fcom
drush en fastcomments
```

## Install manually

Download the module from [drupal.org/project/fcom](https://www.drupal.org/project/fcom) and place it in your site's `modules/custom/fastcomments/` directory. Then enable it with `drush en fastcomments`, or from the admin UI at `Extend` (`/admin/modules`).

<sup>Note!</sup> The module only depends on Drupal core (`user` and `field`). There are no other Drupal modules or libraries required.

Once the module is enabled, head to the `Configuration` section to set up your Tenant ID and API Secret.
