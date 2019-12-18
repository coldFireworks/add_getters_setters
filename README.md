# add_getters_setters

Makes it much easier to add getters and setters for fields of structures.
Done by simply just adding some attributes to the struct and fields, see example below, or look at the source code of the test file.

**Example**

    struct HorseRider {
	    //stuff here
	}
    struct RaceHorse {
	    #[get]
	    name: String,
	    
	    #[get]
	    #[set]
	    speed: i16,

	    #[get]
	    #[get_mut]
	    rider: HorseRider,
	}
With this code, these methods would be generared for you
(provided that you have `#[macro_use] extern crate add_getters_setters;` at the top of `main.rs`):

    impl RaceHorse {
	    pub fn get_name(&self) -> &String {
		    &self.name
	    }
	    
	    pub fn get_speed(&self) -> &i16 {
		    &self.speed
	    }
	    
	    pub fn set_speed(&mut self, v: i16) {
		    self.speed = v;
	    }
	    
	    pub fn get_rider(&self) -> &HorseRider {
		    &self.rider
	    }
	    
	    pub fn get_rider_mut(&mut self) -> &mut HorseRider {
		    &mut self.rider
	    }
	}
	    
