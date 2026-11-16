Fügen Sie das FastComments SDK zur `build.gradle.kts`-Datei Ihrer App hinzu:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Stellen Sie sicher, dass sich das Repsy-Repository in der `settings.gradle.kts` Ihres Projekts befindet:

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