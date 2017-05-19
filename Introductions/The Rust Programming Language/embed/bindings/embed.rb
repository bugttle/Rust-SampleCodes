# $ gem install ffi
# $ ruby embed.rb

require 'ffi'

module Hello
  extend FFI::Library
  ffi_lib '../target/release/libembed.dylib' #=> for Mac
  #ffi_lib '../target/release/libembed.so' #=> for Linux
  attach_function :process, [], :void
end

Hello.process

puts 'done!'
