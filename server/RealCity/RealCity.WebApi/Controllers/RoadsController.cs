using Microsoft.AspNetCore.Mvc;

namespace RealCity.WebApi.Controllers
{
    public static class AsyncExtensions
    {
        public static Task<T> AsFromResult<T>(this T obj) => Task.FromResult(obj);
    }

    public class RoadsDTO
    {
        public IEnumerable<Road>? Roads { get; set; }
    }

    public class Road
    {
        public IEnumerable<Node>? Nodes { get; set; }
    }

    public class Node
    {
        public int Lat { get; set; }
        public int Long { get; set; }
    }

    [ApiController]
    [Route("api/v1.0/[controller]")]
    [Consumes("application/json")]
    public class RoadsController : ControllerBase
    {
        [HttpGet]
        public Task<RoadsDTO> GetRoads() => new RoadsDTO 
        {
            Roads = Enumerable.Range(0, 10).Select(x => new Road
            {
                Nodes = new List<Node>
                {
                    new Node { Lat = 1 + x, Long = 2 + x },
                    new Node { Lat = 3 + x, Long = 5 + x },
                    new Node { Lat = 3 + x, Long = 7 + x },
                }
            })
        }.AsFromResult();

        [HttpGet("{id}")]
        public Task<Road> GetRoad(Guid id) => new Road
        {
            Nodes = new List<Node>
                {
                    new Node { Lat = 1, Long = 2 },
                    new Node { Lat = 3, Long = 5 },
                    new Node { Lat = 3, Long = 7 },
                }
        }.AsFromResult();
    }
}
