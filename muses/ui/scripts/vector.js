//const { DH_UNABLE_TO_CHECK_GENERATOR } = require("constants");

function Vector(x, y) {
	var that = this;
  	
    // Set vector value
  	that.x = x;
  	that.y = y;
  
    // Plus two vectors
    that.plus = function (vector){
    	return new Vector(that.x + vector.x, that.y + vector.y); 
    }
    
    // Minus two vectors
    that.minus = function (vector){
      return new Vector(that.x - vector.x, that.y - vector.y);
    }

    that.mul = function(vector) {
      return new Vector(that.x * vector.x, that.y * vector.y);
    }
    
    that.mul_scalar = function(scalar) {
        return new Vector(that.x * scalar, that.y * scalar);
    }

    // Overriding property 'length'
    Object.defineProperty(that, 'length', {
    	get: function (){
          return Math.sqrt(Math.pow(that.y, 2) + Math.pow(that.x, 2));
    	}
    });
};