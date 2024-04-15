module.exports = function (grunt) {
  grunt.initConfig({
    pkg: grunt.file.readJSON('package.json'),
    cssmin: {
      target: {
        files: {
          './src/index.css': ['./src/index.css'] 
        }
      }
    }
  });

  grunt.loadNpmTasks('grunt-contrib-cssmin');

  grunt.registerTask('minify', ['cssmin']);

  grunt.registerTask('default', ['minify']); 
};
