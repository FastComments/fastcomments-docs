Добавете FastComments SDK към `build.gradle.kts` файла на вашето приложение:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Уверете се, че имате Repsy хранилището в `settings.gradle.kts` на вашия проект:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // други хранилища...
    }
}
```