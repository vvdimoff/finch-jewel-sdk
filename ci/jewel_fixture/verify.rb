# Loaded by CI after building the fixture into finch_jewels_ci_example.<ext> beside this
# file. Proves export! end-to-end: module name, Init symbol, arity 1 and 2, no shared Native.
require_relative "finch_jewels_ci_example"

unless defined?(Finch::Jewels_ci_example)
  abort "FAIL: module Finch::Jewels_ci_example not defined after require"
end
m = Finch::Jewels_ci_example

got = m.triple(2.0)                       # arity 1
abort "FAIL: triple(2.0)=#{got.inspect}, want 6.0" unless got == 6.0

packed = m.pack_pair(1.5, 2.5)            # arity 2
vals = packed.unpack("E*")
abort "FAIL: pack_pair(1.5,2.5).unpack('E*')=#{vals.inspect}, want [1.5, 2.5]" unless vals == [1.5, 2.5]

# SDK must NOT create a shared Finch::Native — jewels live only under Finch::Jewels_<ns>.
abort "FAIL: unexpected shared Finch::Native created by the SDK" if defined?(Finch::Native)

puts "OK finch-jewel-sdk: Finch::Jewels_ci_example loaded; arity 1 and 2 verified; no shared Finch::Native"
