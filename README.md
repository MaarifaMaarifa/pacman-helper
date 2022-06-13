
# pacman-helper

More options for package management in Arch systems.
Holding back packages in arch is one of the things that users do in their systems.  
This can be due to stability issues, especially when a new package release becomes buggy.  

**pacman-helper** provides more options for this kind of scenario, in which you want to pause a package update and also it's dependencies without affecting other programs.  
This is done by checking package's unique dependancies, and printing them out to the console, there after the user can know which dependencies to hold back without breaking  
the system.

## usage
### **Getting unique dependencies**
```sh
# Replace "package-name" with the name of your package
pacman-helper --get-unique-deps "package-name"
```

### **Getting other programs that have common dependencies with the package supplied**
```sh
# Replace "package-name" with the name of your package
pacman-helper --get-pacs-with-same-deps "package-name"
```
