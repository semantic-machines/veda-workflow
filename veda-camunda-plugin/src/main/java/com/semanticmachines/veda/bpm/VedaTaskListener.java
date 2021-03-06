package com.semanticmachines.veda.bpm;

import org.camunda.bpm.engine.delegate.DelegateTask;
import org.camunda.bpm.engine.delegate.TaskListener;
import org.sm.vqueue.VQueue;

import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.util.Properties;
import java.util.UUID;
import java.util.logging.Logger;

/**
 * Task listener to be executed when a user task is entered
 */
public class VedaTaskListener implements TaskListener {

    // Possible events = create, assignment, complete, update, delete, timeout
    public static VQueue queue;

    private final Logger LOGGER = Logger.getLogger(this.getClass().getName());
    public static long callCounter = 0;

    private static final Properties properties = new Properties();
    private static String queue_path;
    private static VedaTaskListener instance = null;

    protected VedaTaskListener() {
    }

    private static void writeDefaultProperties() {
        System.out.println("Writing default properties.");

        queue_path = "./data/queue/camunda-events";

        properties.setProperty("queue_path", queue_path);
        try {
            properties.store(new FileOutputStream("camunda-veda-plugin.properties"), null);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private static void loadProperties() {
        try {
            properties.load(new FileInputStream("camunda-veda-plugin.properties"));

            queue_path = properties.getProperty("queue_path", "");
            System.out.println(properties);
        } catch (IOException e) {
            writeDefaultProperties();
        }
    }

    public static VedaTaskListener getInstance() {
        if (instance == null) {
            instance = new VedaTaskListener();
            loadProperties();
            try {
                queue = new VQueue();
                queue.setNameAndPath("camunda-event", queue_path);
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
        return instance;
    }

    public void notify(DelegateTask delegateTask) {
        callCounter++;
        String event = delegateTask.getEventName();
        LOGGER.info("[" + callCounter + "] Task event: '" + event + "', task: " + delegateTask);

        queue.push(event + "," + delegateTask);
    }

}
