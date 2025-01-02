<script lang="ts">
  import { Chart, Card } from "flowbite-svelte";
</script>

<div class="container">
  <div class="flex flex-row gap-4">
    <Card size="lg">
      <h4 class="text-2xl font-semibold">Users Connected</h4>
      <Chart
        options={{
          chart: {
            type: "area",
            fontFamily: "Inter, sans-serif",
            dropShadow: {
              enabled: false,
            },
            toolbar: {
              show: false,
            },
          },
          fill: {
            type: "gradient",
            gradient: {
              opacityFrom: 0.55,
              opacityTo: 0,
              shade: "#6366f1",
              gradientToColors: ["#6366f1"],
            },
          },
          dataLabels: {
            enabled: false,
          },
          stroke: {
            width: 6,
          },
          grid: {
            show: false,
            strokeDashArray: 4,
            padding: {
              left: 2,
              right: 2,
              top: 0,
            },
          },
          series: [
            {
              name: "Users Connected",
              data: [6500, 6418, 6456, 6526, 6356, 6456],
              color: "#6366f1",
            },
          ],
          xaxis: {
            categories: [
              "01 February",
              "02 February",
              "03 February",
              "04 February",
              "05 February",
              "06 February",
              "07 February",
            ],
            labels: {
              show: false,
            },
            axisBorder: {
              show: false,
            },
            axisTicks: {
              show: false,
            },
          },
          yaxis: {
            show: false,
          },
        }}
      />
    </Card>
    <Card size="lg">
      <h4 class="text-2xl font-semibold">Group Users</h4>
      <Chart
        options={{
          series: [52.8, 26.8, 20.4],
          colors: ["#1C64F2", "#16BDCA", "#9061F9"],
          chart: {
            height: 420,
            width: "100%",
            type: "pie",
          },
          stroke: {
            colors: ["white"],
          },
          plotOptions: {
            pie: {
              labels: {
                show: true,
              },
              size: "100%",
              dataLabels: {
                offset: -25,
              },
            },
          },
          labels: ["Group 1", "Group 2", "Group 3"],
          dataLabels: {
            enabled: true,
            style: {
              fontFamily: "Inter, sans-serif",
            },
          },
          legend: {
            position: "bottom",
            fontFamily: "Inter, sans-serif",
          },
          yaxis: {
            labels: {
              formatter: function (value) {
                return value + "%";
              },
            },
          },
          xaxis: {
            labels: {
              formatter: function (value) {
                return value + "%";
              },
            },
            axisTicks: {
              show: false,
            },
            axisBorder: {
              show: false,
            },
          },
        }}
      />
    </Card>
  </div>
</div>
