Добавете FastComments SDK към файла `build.gradle.kts` на вашето приложение:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Уверете се, че имате хранилището Repsy в `settings.gradle.kts` на вашия проект:

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