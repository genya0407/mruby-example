MRuby::Gem::Specification.new('mruby-example') do |spec|
  
  spec.license = 'MIT'
  spec.authors = 'Yusuke Sangenya'

  system("cd #{__dir__}/example-ext && cargo build --release", exception: true)
  spec.linker.libraries << 'example_ext'
  spec.linker.library_paths << "#{__dir__}/example-ext/target/release"
end
