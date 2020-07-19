import React, { useCallback, useEffect, useState } from "react";
import AddIcon from "@material-ui/icons/Add";
import {
  AppBar,
  Backdrop,
  CircularProgress,
  Container,
  CssBaseline,
  Fab,
  Grid,
  Toolbar,
  Typography,
  makeStyles,
} from "@material-ui/core";

import HostCard from "./HostCard";
import CreateHostDialog from "./CreateHostDialog";

const useStyles = makeStyles((theme) => ({
  root: {
    height: "100%",
  },
  title: {
    marginTop: theme.spacing(3),
    marginBottom: theme.spacing(3),
  },
  grid: {
    width: "100%",
  },
  fab: {
    position: "absolute",
    bottom: theme.spacing(3),
    right: theme.spacing(3),
  },
  backdrop: {
    zIndex: 1300,
  },
}));

interface Host {
  id: number;
  name: string;
  ip_address: string;
  mac_address: string;
}

async function fetchHosts(): Promise<Host[]> {
  const res = await fetch("/api/hosts");
  const data = await res.json();
  return data;
}

async function createHost(
  name: string,
  ipAddress: string,
  macAddress: string
): Promise<Host> {
  const res = await fetch("/api/hosts", {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify({
      name: name,
      ip_address: ipAddress,
      mac_address: macAddress,
    }),
  });

  return await res.json();
}

async function wakeHost(id: number) {
  await fetch(`/api/hosts/${id}/wake`, {
    method: "POST",
  });
}

function App() {
  const classes = useStyles();

  const [hosts, setHosts] = useState<Host[]>();
  const [showCreateHost, setShowCreateHost] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    setLoading(true);
    fetchHosts().then((hosts) => {
      setHosts(hosts);
      setLoading(false);
    });
  }, []);

  const saveHost = useCallback(
    async (name: string, ipAddress: string, macAddress: string) => {
      setShowCreateHost(false);
      setLoading(true);
      await createHost(name, ipAddress, macAddress);
      setHosts(await fetchHosts());
      setLoading(false);
    },
    []
  );

  return (
    <div className={classes.root}>
      <CssBaseline />
      <AppBar position="sticky">
        <Toolbar>
          <Typography variant="h6">Vigil</Typography>
        </Toolbar>
      </AppBar>
      <Container>
        <div>
          <Typography variant="h2" className={classes.title}>
            My Hosts
          </Typography>
          <Grid container spacing={2} className={classes.grid}>
            {hosts &&
              hosts.map((host) => (
                <Grid key={host.id} item>
                  <HostCard
                    name={host.name}
                    ipAddress={host.ip_address}
                    macAddress={host.mac_address}
                    onWake={() => wakeHost(host.id)}
                  />
                </Grid>
              ))}
          </Grid>
        </div>
      </Container>
      <CreateHostDialog
        open={showCreateHost}
        onSave={saveHost}
        onCancel={() => setShowCreateHost(false)}
      />
      <Fab
        color="secondary"
        className={classes.fab}
        onClick={() => setShowCreateHost(true)}
      >
        <AddIcon />
      </Fab>
      <Backdrop open={loading} className={classes.backdrop}>
        <CircularProgress color="secondary" size={60} />
      </Backdrop>
    </div>
  );
}

export default App;
