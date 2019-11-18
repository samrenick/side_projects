<template>
  <div class="container-fluid">
    <div class="row">
      <div class="col-md">
        <div class="form-group">
          <label for="input_filename">Input filename</label>
          <input
            type="text"
            class="form-control"
            id="input-filename"
            placeholder="like /tmp/temp.json"
            v-model.lazy="input_filename"
          />
        </div>
        <button type="submit" class="btn btn-primary" @click="getResults">
          Submit
        </button>
      </div>
      <div class="col-md">
        <div class="form-group">
          <label for="output_filename">Output filename</label>
          <input
            type="text"
            class="form-control"
            id="output-filename"
            v-model.lazy="output_filename"
          />
        </div>
      </div>
      <div class="col-md">
        <div class="form-group">
          <label for="output_filename">Column to add</label>
          <input
            type="text"
            class="form-control"
            id="output-filename"
            v-model="column_to_add"
          />
        </div>
      </div>
      <div class="col-md">
        <div class="form-group">
          <label for="output_filename">Filter</label>
          <input
            type="text"
            class="form-control"
            id="filter"
            v-model.lazy="filter"
          />
        </div>
      </div>
    </div>
    <div class="row">
      <div class="col-md">
        <pre
          v-if="result.length > 0"
          style="background-color: #3e4049; color: #d9ebe7"
        ><code class="json">{{result}}</code></pre>
      </div>
    </div>
  </div>
</template>

<script>
var g = function(v) {
  console ? console.log(v) : null;
};
import axios from "axios";
export default {
  name: "SwissJsonKnife",
  props: {
    msg: String
  },
  data: function() {
    return {
      filter: "",
      input_filename: "/tmp/input.json",
      output_filename: "/tmp/output.json",
      columns: [],
      column_to_add: "",
      result: ""
    };
  },
  created: function() {
    g("whatever");
  },
  methods: {
    getResults() {
      let input_filename = this.input_filename.split("/").join("-");
      // let output_filename = this.output_filename.split("/").join("-");
      let url = `http://newpepe:8088/parse_json/filename/${input_filename}/columns/-/filter/-`;
      let config = {
        "Content-Type": "application/json"
      };
      axios
        .get(url, null, config)
        .then(response => {
          console.log(response);
        })
        .catch(error => {
          console.log(error);
        });
    }
  }
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.container-fluid {
  margin-top: 1em;
}
</style>
