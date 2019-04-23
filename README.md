# emptypipe

Run a command if stdin is empty

```shell
  echo | emptypipe echo 2
 ```
 
 Run a command if stdin is not empty

```shell
  echo 1 | emptypipe -i echo 2
