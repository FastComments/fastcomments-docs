Add the FastComments SDK to your app's `build.gradle.kts` file:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Make sure you have the Repsy repository in your project's `settings.gradle.kts`:

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