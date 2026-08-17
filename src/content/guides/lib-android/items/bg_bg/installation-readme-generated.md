---
Добавете FastComments SDK към вашия файл `build.gradle.kts`:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Уверете се, че имате репозитория Repsy в `settings.gradle.kts` на вашия проект:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```
---