# Montgomery-Benchmarks

Recently, in [a blog post](https://hackmd.io/@Ingonyama/Barret-Montgomery) Yuval Domb describes a subtle, but remarkably clever optimisation that allows us to compute the montgomery product of two $n$ word integers using $2n^2 + 1$ multiplication operations (as opposed to the well known CIOS algorithm which uses $2n^2 + n$ multiplications.
Although an improvement in theory, when compared with exisitng code for Montgomery multiplication, we were unable to manifest this speedup using the provided code.
This [post](https://randomwalks.xyz/posts/mont_mult/) is a deep dive into why this could be? All the code used for benchmarking is found in this repository.


