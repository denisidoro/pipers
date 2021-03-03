# Pipers

Use pipe expressions in your PromQL queries or code!

For example, the following query...
```promql
sum(irate(node_cpu_seconds_total{instance=~"$node:$port",job=~"$job"}(node_cpu_seconds_total){mode='idle'}[5m](node_cpu_seconds_total{instance=~"$node:$port",job=~"$job"}(node_cpu_seconds_total)))) / count(count(node_cpu_seconds_total{instance=~"$node:$port",job=~"$job"}(node_cpu_seconds_total)) by (cpu))
```
...can be written as:
```promql
cpuSeconds = node_cpu_seconds_total
  | x -> x{instance=~"$node:$port",job=~"$job"}

cpuCount = cpuSeconds 
  | s -> count(s) by (cpu)
  | count

cpuIdle = cpuSeconds
  | s -> s{mode='idle'}[5m]
  | irate
  | sum

cpuIdle / cpuCount
```

## Syntax and live playground

Check [this page](https://denisidoro.github.io/pipers/) to try it out!

## Using it inside your IDE 

### add the binary to your `$PATH`
```bash
git clone https://github.com/denisidoro/pipers
cargo build --release
```
### setup your IDE accordingly

E.g., in VSCode you can use the [Filter Text](https://marketplace.visualstudio.com/items?itemName=yhirose.FilterText) extension:

![Demo](https://user-images.githubusercontent.com/3226564/109806044-f9f08380-7c02-11eb-9429-92d26ee7084c.gif)
