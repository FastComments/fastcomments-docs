Добавьте FastComments SDK в файл `build.gradle.kts` вашего приложения:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Убедитесь, что у вас есть репозиторий Repsy в `settings.gradle.kts` вашего проекта:

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